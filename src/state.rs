use std::collections::HashMap;

use cgmath::Vector2;
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, window::Window};

use crate::{ecs_utils::ecs, ecs_utils::systems, rendering, shapes};
use rendering::{camera, model, texture, instance};
use shapes::shape;

use ecs::{TextureIndex, Ecs};
use camera::{Camera, CameraUniform};
use instance::InstanceModel;
use model::{DrawModel, Model, Vertex};
use shape::ShapeEnum;

pub struct State {
    instance_map: HashMap<(ShapeEnum, TextureIndex), (Model, Vec<InstanceModel>)>,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    config: wgpu::SurfaceConfiguration,
    camera_bind_group: wgpu::BindGroup,
    depth_texture: texture::Texture,
    texture_map: Vec<Box<[u8]>>,
    world_size: (f32, f32),
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    camera: Camera,
    window: Window,
    ecs: Ecs,
}

impl State {
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(wgpu::TextureFormat::is_srgb)
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // texture stuff
        let happy_tree_texture = rendering::resources::load_binary("happy-tree.png")
            .await
            .unwrap();
        let gray_texture = rendering::resources::load_binary("gray.png")
            .await
            .unwrap();

        let texture_map: Vec<Box<[u8]>> = vec![happy_tree_texture.into(), gray_texture.into()];
        
        // entity stuff
        let instance_map: HashMap<(ShapeEnum, TextureIndex), (Model, Vec<InstanceModel>)> = HashMap::new();

        let mut ecs = Ecs::new();

        let fixed_point = ecs.add_fixed_point(Vector2::new(0.0, 2.0));
        let cube1 = ecs.add_cube(Vector2::new(1.0, 2.0), Vector2::new(0.0, 0.0), 0.1, 0.5, 0.5, 0);
        ecs.add_spring(20, 0.01, 0.2, 1.0, 20.0, fixed_point, cube1, 1);

        let cube2 = ecs.add_cube(Vector2::new(2.0, 2.0), Vector2::new(0.0, 0.0), 0.1, 0.5, 0.5, 0);
        ecs.add_spring(20, 0.01, 0.2, 1.0, 20.0, cube1, cube2, 1);

        let cube3 = ecs.add_cube(Vector2::new(3.0, 2.0), Vector2::new(0.0, 0.0), 0.1, 0.5, 0.5, 0);
        ecs.add_spring(20, 0.01, 0.2, 1.0, 20.0, cube2, cube3, 1);

        let cube4 = ecs.add_cube(Vector2::new(4.0, 2.0), Vector2::new(0.0, 0.0), 0.1, 0.5, 0.5, 0);
        ecs.add_spring(20, 0.01, 0.2, 1.0, 20.0, cube3, cube4, 1);

        // camera stuff
        let camera = Camera::new(2.0, config.width as f32 / config.height as f32);
        let world_size = camera.get_world_size();

        let camera_uniform = CameraUniform::from_camera(&camera);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader.wgsl"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[model::ModelVertex::desc(), InstanceModel::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        Self {
            instance_map,
            size,
            render_pipeline,
            config,
            camera_bind_group,
            depth_texture,
            texture_map,
            world_size,
            surface,
            device,
            queue,
            camera,
            window,
            ecs,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.camera.update_aspect(
                self.config.width as f32 / self.config.height as f32
            );
            self.world_size = self.camera.get_world_size();
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture = texture::Texture::create_depth_texture(
                &self.device, &self.config, "depth_texture"
            );
        }
    }
    pub fn input(&mut self, _: &WindowEvent) -> bool {
        // self.player.process_events(event)
        false
    }

    pub fn update(&mut self, dt: instant::Duration) {
        let layout = self.render_pipeline.get_bind_group_layout(0);

        // apply physics step
        systems::physics_systems::physics_system(
            &self.ecs.spring_force_components,
            &self.ecs.connection_components,
            &mut self.ecs.position_components,
            &mut self.ecs.physics_components,
            &dt
        );

        // update connection instances
        systems::rendering_systems::connection_system(
            &self.ecs.connection_components,
            &mut self.ecs.position_components,
            &mut self.ecs.rotation_components,
            &mut self.ecs.size_components,
        );

        // update rendering instances
        systems::rendering_systems::instance_system(
            &mut self.instance_map,
            &self.texture_map,
            &self.ecs.shape_components,
            &self.ecs.position_components,
            &self.ecs.rotation_components,
            &self.ecs.texture_components,
            &self.ecs.size_components,
            &self.device,
            &self.queue,
            &layout,
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut instance_buffers: Vec<wgpu::Buffer> = Vec::with_capacity(self.instance_map.len());
        for (_, instances) in self.instance_map.values() {
            instance_buffers.push(self.device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(instances),
                    usage: wgpu::BufferUsages::VERTEX,
                },
            ));
        }

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 1.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            render_pass.set_pipeline(&self.render_pipeline);

            for (instance_buffer, (model, instances)) in
                core::iter::zip(instance_buffers.iter(), self.instance_map.values())
            {
                render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

                render_pass.draw_model_instanced(
                    model,
                    0..instances.len() as u32,
                    &self.camera_bind_group,
                );
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
