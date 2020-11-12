use winit::{window::Window, event_loop::{EventLoop}};
pub struct WGPUData {
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,
    surface: wgpu::Surface,
    render_pipeline: wgpu::RenderPipeline,
    pipeline_layout: wgpu::PipelineLayout,
    shader_module: (wgpu::ShaderModule, wgpu::ShaderModule)
}

pub struct MainGame {
    wgpu_data: WGPUData,
    window: Window,
    event_loop: EventLoop<()>
}

impl MainGame 
{
    pub async fn new() -> MainGame {
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let event_loop = EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();
        let size = window.inner_size();
        let surface = unsafe { instance.create_surface(&window) };
        
        let adapter = MainGame::make_adapter(&instance, &surface).await;
        let (device, queue) = MainGame::request_device(&adapter).await;
        let pipeline_layout = MainGame::make_pipeline_layout(&device);
        let (shader_module, render_pipeline) = MainGame::make_render_pipeline(&device, &pipeline_layout);
        MainGame{
            event_loop, window,
            wgpu_data: WGPUData {
                device, queue, adapter, surface, render_pipeline, pipeline_layout, shader_module
            }
        }
    }
    async fn make_adapter(instance: &wgpu::Instance, surface: &wgpu::Surface) -> wgpu::Adapter {
        instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            // Request an adapter which can render to our surface
            compatible_surface: Some(surface),
        })
        .await
        .expect("Failed to find an appropiate adapter")
    }
    async fn request_device(adapter: &wgpu::Adapter) -> (wgpu::Device, wgpu::Queue) {
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                shader_validation: true,
            },
            None,
        )
        .await
        .expect("Failed to create device")
    }
    fn make_render_pipeline(device: &wgpu::Device, pipeline_layout: &wgpu::PipelineLayout) -> ((wgpu::ShaderModule,wgpu::ShaderModule),wgpu::RenderPipeline) {
        let vs_module=device.create_shader_module(wgpu::include_spirv!("../shaders/shader.vert.spv"));
        let fs_module=device.create_shader_module(wgpu::include_spirv!("../shaders/shader.frag.spv"));
        let rp = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            // Use the default rasterizer state: no culling, no depth bias
            rasterization_state: None,
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::TextureFormat::Bgra8UnormSrgb.into()],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });
        ((vs_module, fs_module), rp)
    }
    fn make_render_pass<'a>(encoder: &'a wgpu::CommandEncoder, render_pipeline: &'a wgpu::RenderPipeline, view: &'a wgpu::TextureView) -> wgpu::RenderPass<'a> {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(render_pipeline);
        // rpass.draw(0..3, 0..1);
        rpass
    }
    fn make_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        })
    }
}