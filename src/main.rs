use eframe::egui;
use std::sync::Arc;

/// 塔防精灵AI - 主应用结构
/// 从 C# WinForms Form1.Designer.cs 转换而来
///
/// 组件对照表:
///   toolStrip1        → 顶部工具栏 (语言/主题切换 + 用户信息)
///   splitContainer1   → 左右分栏布局
///   groupBox1         → 左侧信息面板 (关卡 + 窗口信息)
///   groupBox2         → 火炮按钮网格 (14个按钮 + VS标签)
///   pictureBox1       → 图标拖拽区域 (查找窗口)
///   pictureBox2       → 截图预览区域
///   button15/16/17    → 动态截图/实时状态/提取图标
///   comboBox1         → AI模型选择下拉框
///   webView21         → WebView2浏览器 (egui中用文本区域替代)
///   statusStrip1      → 底部状态栏
struct TfjlAiApp {
    // ===== 顶部工具栏状态 (toolStrip1) =====
    language: String,       // 当前语言: "中文" / "ENGLISH"
    theme: String,          // 当前主题: "白天" / "夜晚"
    user_name: String,      // 用户名

    // ===== 左侧面板 - 关卡信息 (label2 "关卡：" + label3) =====
    level: String,

    // ===== 左侧面板 - 火炮按钮 (groupBox2, button1~button14) =====
    // 原始布局: 4行多列的不规则网格，简化为 2行×7列
    cannon_buttons: [[String; 7]; 2],

    // ===== 左侧面板 - 窗口信息 (groupBox1 右侧) =====
    window_handle: String,  // 窗口句柄 (textBox1)
    window_name: String,    // 窗口名称 (textBox2)
    window_class: String,   // 窗口类名 (textBox3)

    // ===== 左侧面板 - 截图预览 (pictureBox2, 630x302) =====
    screenshot_available: bool,

    // ===== 右侧面板 - AI模型选择 (comboBox1) =====
    ai_models: Vec<String>,
    selected_ai_model: usize,

    // ===== 右侧面板 - WebView 替代 (webView21) =====
    webview_url: String,

    // ===== 底部状态栏 (statusStrip1) =====
    status_message: String,
}

impl Default for TfjlAiApp {
    fn default() -> Self {
        Self {
            language: "中文".to_string(),
            theme: "白天".to_string(),
            user_name: "用户".to_string(),

            level: "000".to_string(),

            cannon_buttons: [
                [
                    "火炮1".into(), "火炮2".into(), "火炮3".into(),
                    "火炮4".into(), "火炮5".into(), "火炮6".into(),
                    "火炮7".into(),
                ],
                [
                    "火炮8".into(), "火炮9".into(), "火炮10".into(),
                    "火炮11".into(), "火炮12".into(), "火炮13".into(),
                    "火炮14".into(),
                ],
            ],

            window_handle: String::new(),
            window_name: String::new(),
            window_class: String::new(),

            screenshot_available: false,

            ai_models: vec![
                "豆包".into(), "deepseek".into(), "千问".into(),
                "文心一言".into(), "智谱清言".into(), "讯飞星火".into(),
                "Kimi".into(), "腾讯混元".into(),
            ],
            selected_ai_model: 0,

            webview_url: String::new(),

            status_message: "就绪".to_string(),
        }
    }
}

impl eframe::App for TfjlAiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        // ===== 底部状态栏 =====
        egui::TopBottomPanel::bottom("status_bar")
            .default_height(22.0)
            .show(ctx, |ui| {
                self.status_bar(ui);
            });

        // ===== 顶部工具栏 =====
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.toolbar(ui);
        });

        // ===== 左侧面板 (对应 splitContainer1.Panel1, 638px) =====
        egui::SidePanel::left("left_panel")
            .default_width(638.0)
            .width_range(400.0..=800.0)
            .resizable(true)
            .show(ctx, |ui| {
                self.left_panel(ui);
            });

        // ===== 右侧面板 (对应 splitContainer1.Panel2, 278px) =====
        egui::CentralPanel::default().show(ctx, |ui| {
            self.right_panel(ui);
        });
    }
}

impl TfjlAiApp {
    /// 应用主题 (白天/夜晚)
    fn apply_theme(&self, ctx: &egui::Context) {
        match self.theme.as_str() {
            "夜晚" => ctx.set_visuals(egui::Visuals::dark()),
            _ => ctx.set_visuals(egui::Visuals::light()),
        }
    }

    /// 顶部工具栏 - 对应 toolStrip1
    /// [中文/ENGLISH] [白天/夜晚] .......................... [user] [用户]
    fn toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // 语言切换 (toolStripButton1 / toolStripButton2)
            if ui.button(&self.language).clicked() {
                self.language = if self.language == "中文" {
                    "ENGLISH".into()
                } else {
                    "中文".into()
                };
                self.status_message = format!("语言已切换为: {}", self.language);
            }

            // 主题切换 (toolStripButton3)
            if ui.button(&self.theme).clicked() {
                self.theme = if self.theme == "白天" {
                    "夜晚".into()
                } else {
                    "白天".into()
                };
                self.status_message = format!("主题已切换为: {}", self.theme);
            }

            // 右侧: user 标签 + 用户名 (toolStripLabel1 + 用户)
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.strong(&self.user_name);
                ui.label("user");
            });
        });
    }

    /// 左侧面板 - 对应 splitContainer1.Panel1 (638px)
    /// 从上到下: groupBox1 → pictureBox2 → button15/16/17
    fn left_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // ===== groupBox1 - 信息面板 (631x173) =====
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(300.0);
                ui.vertical(|ui| {
                    // 关卡信息行 (label2 + label3)
                    ui.horizontal(|ui| {
                        ui.label("关卡：");
                        ui.add(egui::TextEdit::singleline(&mut self.level).desired_width(60.0));
                    });

                    ui.add_space(4.0);

                    // ===== groupBox2 - 火炮按钮网格 (308x128) =====
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.vertical(|ui| {
                            // 2行 × 7列 火炮按钮
                            for row in 0..2 {
                                ui.horizontal(|ui| {
                                    for col in 0..7 {
                                        let label = self.cannon_buttons[row][col].clone();
                                        let btn = egui::Button::new(&label).min_size(egui::vec2(61.0, 25.0));
                                        if ui.add(btn).clicked() {
                                            self.status_message = format!("点击了: {}", label);
                                        }
                                    }
                                });
                            }

                            // VS 标签 (label1)
                            ui.add_space(2.0);
                            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                                ui.strong("VS");
                            });
                        });
                    });

                    ui.add_space(4.0);

                    // 窗口信息区域 (pictureBox1 + textBox1/2/3)
                    ui.horizontal(|ui| {
                        // pictureBox1 - 图标拖拽区域 (34x33)
                        // 原始事件: MouseDown/MouseMove/MouseUp (拖拽查找窗口)
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(34.0, 33.0),
                            egui::Sense::click_and_drag(),
                        );
                        ui.painter().rect_filled(rect, 4.0, egui::Color32::LIGHT_GRAY);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "🎯",
                            egui::FontId::proportional(18.0),
                            egui::Color32::BLACK,
                        );
                        if response.clicked() {
                            self.status_message = "图标点击 - 查找窗口功能".into();
                        }

                        ui.add_space(8.0);

                        // 窗口信息输入框
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([68.0, 20.0], egui::Label::new("窗口句柄："));
                                ui.add_sized([199.0, 23.0], egui::TextEdit::singleline(&mut self.window_handle));
                            });
                            ui.horizontal(|ui| {
                                ui.add_sized([68.0, 20.0], egui::Label::new("窗口名称："));
                                ui.add_sized([199.0, 23.0], egui::TextEdit::singleline(&mut self.window_name));
                            });
                            ui.horizontal(|ui| {
                                ui.add_sized([68.0, 20.0], egui::Label::new("窗口类名："));
                                ui.add_sized([199.0, 23.0], egui::TextEdit::singleline(&mut self.window_class));
                            });
                        });
                    });
                });
            });

            ui.add_space(4.0);

            // ===== pictureBox2 - 截图预览区域 (630x302) =====
            let screenshot_height = 200.0;
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                ui.set_min_height(screenshot_height);
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                    if self.screenshot_available {
                        ui.label(egui::RichText::new("截图已加载").color(egui::Color32::GREEN));
                    } else {
                        ui.label(
                            egui::RichText::new("截图预览区域 (630 × 302)\nSizeMode: StretchImage")
                                .color(egui::Color32::GRAY),
                        );
                    }
                });
            });

            ui.add_space(4.0);

            // ===== 操作按钮行 =====
            // button15 "动态截图" (21,490) | button16 "实时状态" (110,490) | button17 "提取图标" (564,490)
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("动态截图").min_size(egui::vec2(69.0, 30.0))).clicked() {
                    self.status_message = "动态截图 - 功能待实现".into();
                }
                if ui.add(egui::Button::new("实时状态").min_size(egui::vec2(69.0, 30.0))).clicked() {
                    self.status_message = "实时状态 - 功能待实现".into();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(egui::Button::new("提取图标").min_size(egui::vec2(69.0, 30.0))).clicked() {
                        self.status_message = "提取图标 - 功能待实现".into();
                    }
                });
            });
        });
    }

    /// 右侧面板 - 对应 splitContainer1.Panel2 (278px)
    /// 从上到下: comboBox1 → webView21
    fn right_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // ===== comboBox1 - AI模型选择 =====
            // Items: 豆包, deepseek, 千问, 文心一言, 智谱清言, 讯飞星火, Kimi, 腾讯混元
            egui::ComboBox::from_label("AI模型")
                .selected_text(&self.ai_models[self.selected_ai_model])
                .show_ui(ui, |ui| {
                    for (i, model) in self.ai_models.iter().enumerate() {
                        ui.selectable_value(&mut self.selected_ai_model, i, model);
                    }
                });

            ui.add_space(4.0);

            // ===== webView21 - WebView2 浏览器区域 (278x500) =====
            // egui 无原生 WebView，此处为替代显示
            // 实际项目可集成 wry/webview 库实现完整浏览器功能
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_height(300.0);

                // URL 输入栏
                ui.horizontal(|ui| {
                    ui.label("URL:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.webview_url)
                            .hint_text("输入网址...")
                            .desired_width(f32::INFINITY),
                    );
                });

                ui.add_space(4.0);

                // WebView 内容占位区域
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.add_space(40.0);
                        ui.label(
                            egui::RichText::new("WebView2 浏览器区域")
                                .size(16.0)
                                .color(egui::Color32::GRAY),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("egui 中无原生 WebView 组件\n可集成 wry/webview 库实现完整浏览器功能")
                                .size(12.0)
                                .color(egui::Color32::DARK_GRAY),
                        );
                        ui.add_space(40.0);
                    });
                });
            });
        });
    }

    /// 底部状态栏 - 对应 statusStrip1
    fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(&self.status_message).size(12.0));
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("TFJLai - 塔防精灵AI")
            .with_inner_size([922.0, 574.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "塔防精灵AI",
        options,
        Box::new(|cc| {
            // ========== 【核心：中文正常显示字体设置】 ==========
            let mut fonts = egui::FontDefinitions::default();

            // 安装微软雅黑
            fonts.font_data.insert(
                "msyh".to_string(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "C:/Windows/Fonts/msyh.ttc"
                )))
            );

            // 设置为全局默认字体
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
                .insert(0, "msyh".to_string());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
                .insert(0, "msyh".to_string());

            cc.egui_ctx.set_fonts(fonts);
            // ====================================================

            Ok(Box::new(TfjlAiApp::default()))
        }),
    )
}
