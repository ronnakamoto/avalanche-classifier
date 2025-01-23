use eframe::egui;
use poll_promise::Promise;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SnowTexture {
    granular: bool,
    blocky: bool,
    fluffy: bool,
    density: String,  // "low"|"medium"|"high"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct MovementPattern {
    starting_width: String,  // "point"|"wide"|"undefined"
    propagation: String,     // "fan"|"linear"|"chaotic"|"none"
    vertical_movement: bool, // true for significant vertical displacement
    lateral_spread: bool,    // true for significant sideways spread
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TerrainFeatures {
    slope_angle: Option<String>,
    surface_roughness: String,  // "smooth"|"rough"|"variable"
    anchoring_points: bool,     // trees, rocks, etc.
    convex_rollover: bool,      // terrain rolls over
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VisualCharacteristics {
    powder_cloud: bool,
    fracture_line: bool,
    fracture_depth: Option<String>,  // "shallow"|"deep"|"variable"
    point_release: bool,
    debris_pattern: String,
    snow_texture: SnowTexture,
    movement_pattern: MovementPattern,
    terrain: TerrainFeatures,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AvalancheAnalysis {
    avalanche_present: bool,
    avalanche_type: String,
    confidence_level: f32,
    terrain_features: Vec<String>,
    visual_characteristics: VisualCharacteristics,
}

struct AvalancheClassifier {
    openai_api_key: String,
    image_data: Option<ImageData>,
    promise: Option<Promise<anyhow::Result<AvalancheAnalysis>>>,
    result: Option<AvalancheAnalysis>,
    error: Option<String>,
}

struct ImageData {
    bytes: Vec<u8>,
    texture: Option<egui::TextureHandle>,
}

impl AvalancheClassifier {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_apple_style(&cc.egui_ctx);
        Self {
            openai_api_key: String::new(),
            image_data: None,
            promise: None,
            result: None,
            error: None,
        }
    }

    fn load_image(&mut self, ctx: &egui::Context, bytes: Vec<u8>) {
        if let Ok(image) = image::load_from_memory(&bytes) {
            let rgba = image.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let pixels = rgba.into_vec();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);
            
            self.image_data = Some(ImageData {
                bytes,
                texture: Some(ctx.load_texture(
                    "uploaded-image",
                    color_image,
                    egui::TextureOptions::LINEAR
                )),
            });
        }
    }
}

fn setup_apple_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::light();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);
    style.text_styles = [
        (
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();
    ctx.set_style(style);
}

impl eframe::App for AvalancheClassifier {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Clean, minimal container with elegant spacing
                egui::Frame::none()
                    .inner_margin(egui::vec2(24.0, 16.0))
                    .show(ui, |ui| {
                    // Refined color palette
                    let accent_color = egui::Color32::from_rgb(0, 122, 255);  // iOS blue
                    let success_color = egui::Color32::from_rgb(52, 199, 89); // iOS green
                    let warning_color = egui::Color32::from_rgb(255, 149, 0); // iOS orange
                    let danger_color = egui::Color32::from_rgb(255, 59, 48);  // iOS red
                    let muted_color = egui::Color32::from_rgb(142, 142, 147); // iOS gray

                    ui.vertical_centered_justified(|ui| {
                        ui.label(
                            egui::RichText::new("Avalanche Detection and Risk Analyzer")
                                .size(20.0)
                                .strong()
                        );
                        ui.add_space(16.0);

                        // API Key Input
                        ui.label("OpenAI API Key");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.openai_api_key)
                                .password(true)
                                .hint_text("Enter your OpenAI API key")
                        );
                        ui.add_space(16.0);

                        // Upload Button
                        if ui.button("📁 Upload Mountain Image").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Image", &["png", "jpg", "jpeg", "webp"])
                                .pick_file()
                            {
                                if let Ok(bytes) = std::fs::read(&path) {
                                    self.load_image(ctx, bytes);
                                }
                            }
                        }

                        // Image Preview
                        if let Some(image_data) = &self.image_data {
                            if let Some(texture) = &image_data.texture {
                                ui.add_space(16.0);
                                let max_size = egui::vec2(400.0, 400.0);
                                let image = egui::Image::new(texture)
                                    .fit_to_exact_size(max_size)
                                    .maintain_aspect_ratio(true);

                                egui::Frame::group(ui.style())
                                    .inner_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.add(image);
                                    });
                            }
                        }

                        ui.add_space(16.0);

                        // Analysis Button
                        let button = egui::Button::new(
                            egui::RichText::new("Analyze Image")
                                .color(egui::Color32::WHITE)
                                .size(16.0)
                        )
                        .fill(egui::Color32::from_rgb(0, 122, 255))
                        .rounding(6.0);

                        let api_ready = !self.openai_api_key.is_empty() && self.image_data.is_some();
                        if ui.add_enabled(api_ready, button).clicked() {
                            let api_key = self.openai_api_key.clone();
                            let image_bytes = self.image_data.as_ref().unwrap().bytes.clone();
                            
                            self.promise = Some(Promise::spawn_thread("classify", move || {
                                tokio::runtime::Runtime::new()
                                    .unwrap()
                                    .block_on(async {
                                        classify_image(&api_key, &image_bytes).await
                                    })
                            }));
                        }

                        // Loading and Results
                        if let Some(promise) = &self.promise {
                            match promise.ready() {
                                Some(Ok(result)) => {
                                    self.result = Some(result.clone());
                                    self.error = None;
                                    self.promise = None;
                                }
                                Some(Err(err)) => {
                                    self.error = Some(err.to_string());
                                    self.result = None;
                                    self.promise = None;
                                }
                                None => {
                                    ui.horizontal(|ui| {
                                        ui.spinner();
                                        ui.label("Analyzing terrain features...");
                                    });
                                }
                            }
                        }

                        // Display Results
                        if let Some(result) = &self.result {
                            ui.add_space(16.0);
                            let confidence_color = if result.confidence_level > 80.0 {
                                success_color
                            } else if result.confidence_level > 50.0 {
                                warning_color
                            } else {
                                danger_color
                            };

                            let (type_text, type_color) = match result.avalanche_type.as_str() {
                                "powder" => ("Powder Avalanche", warning_color),
                                "loose-snow" => ("Loose Snow Avalanche", warning_color),
                                "slab" => ("Slab Avalanche", danger_color),
                                "none" => ("No Avalanche Risk", success_color),
                                _ => ("Unknown Type", muted_color),
                            };

                            ui.vertical_centered(|ui| {
                                ui.add_space(8.0);
                                ui.label(
                                    egui::RichText::new(type_text)
                                        .size(24.0)
                                        .color(type_color)
                                        .strong()
                                );
                                
                                // Elegant confidence indicator
                                ui.add_space(8.0);
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    ui.label(
                                        egui::RichText::new("Confidence")
                                            .size(14.0)
                                            .color(muted_color)
                                    );
                                    ui.add_space(4.0);
                                    ui.label(
                                        egui::RichText::new(format!("{:.0}%", result.confidence_level))
                                            .size(14.0)
                                            .color(confidence_color)
                                            .strong()
                                    );
                                    ui.add_space(8.0);
                                    // Refined progress bar
                                    let progress = result.confidence_level / 100.0;
                                    ui.add(
                                        egui::ProgressBar::new(progress)
                                            .desired_width(120.0)
                                            .fill(confidence_color)
                                    );
                                });
                                ui.add_space(16.0);
                            });

                            // Analysis section with refined layout
                            ui.columns(2, |columns| {
                                // Snow Analysis Column
                                columns[0].group(|ui| {
                                    ui.set_min_width(240.0);
                                    ui.label(
                                        egui::RichText::new("Snow Analysis")
                                            .size(16.0)
                                            .strong()
                                    );
                                    ui.add_space(8.0);
                                    
                                    let snow = &result.visual_characteristics.snow_texture;
                                    // Texture indicators with pills
                                    ui.horizontal(|ui| {
                                        if snow.granular {
                                            ui.add(pill_label("Granular", accent_color));
                                        }
                                        if snow.blocky {
                                            ui.add(pill_label("Blocky", accent_color));
                                        }
                                        if snow.fluffy {
                                            ui.add(pill_label("Fluffy", accent_color));
                                        }
                                    });
                                    
                                    ui.add_space(4.0);
                                    // Density indicator
                                    let density_color = match snow.density.as_str() {
                                        "low" => success_color,
                                        "medium" => warning_color,
                                        "high" => danger_color,
                                        _ => muted_color,
                                    };
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new("Density")
                                                .size(13.0)
                                                .color(muted_color)
                                        );
                                        ui.add_space(4.0);
                                        ui.label(
                                            egui::RichText::new(&snow.density)
                                                .size(13.0)
                                                .color(density_color)
                                                .strong()
                                        );
                                    });

                                    ui.add_space(12.0);
                                    // Movement pattern section
                                    ui.label(
                                        egui::RichText::new("Movement Pattern")
                                            .size(16.0)
                                            .strong()
                                    );
                                    ui.add_space(8.0);
                                    
                                    let movement = &result.visual_characteristics.movement_pattern;
                                    info_row(ui, "Initial Release", &movement.starting_width, muted_color, 13.0);
                                    info_row(ui, "Propagation", &movement.propagation, muted_color, 13.0);
                                    
                                    ui.add_space(4.0);
                                    ui.horizontal(|ui| {
                                        if movement.vertical_movement {
                                            ui.add(pill_label("Vertical", accent_color));
                                        }
                                        if movement.lateral_spread {
                                            ui.add(pill_label("Lateral", accent_color));
                                        }
                                    });
                                });

                                // Terrain Analysis Column
                                columns[1].group(|ui| {
                                    ui.set_min_width(240.0);
                                    ui.label(
                                        egui::RichText::new("Terrain Analysis")
                                            .size(16.0)
                                            .strong()
                                    );
                                    ui.add_space(8.0);
                                    
                                    let terrain = &result.visual_characteristics.terrain;
                                    if let Some(angle) = &terrain.slope_angle {
                                        let angle_color = if angle.starts_with("steep") {
                                            danger_color
                                        } else if angle.starts_with("moderate") {
                                            warning_color
                                        } else {
                                            success_color
                                        };
                                        info_row(ui, "Slope", angle, angle_color, 13.0);
                                    }

                                    info_row(ui, "Surface", &terrain.surface_roughness, muted_color, 13.0);
                                    
                                    ui.add_space(4.0);
                                    ui.horizontal(|ui| {
                                        if terrain.anchoring_points {
                                            ui.add(pill_label("Anchoring Points", accent_color));
                                        }
                                        if terrain.convex_rollover {
                                            ui.add(pill_label("Convex Rollover", accent_color));
                                        }
                                    });

                                    if !result.terrain_features.is_empty() {
                                        ui.add_space(12.0);
                                        ui.label(
                                            egui::RichText::new("Additional Observations")
                                                .size(16.0)
                                                .strong()
                                        );
                                        ui.add_space(8.0);
                                        
                                        for feature in &result.terrain_features {
                                            ui.horizontal(|ui| {
                                                ui.label(
                                                    egui::RichText::new("•")
                                                        .size(13.0)
                                                        .color(muted_color)
                                                );
                                                ui.add_space(4.0);
                                                ui.label(
                                                    egui::RichText::new(feature)
                                                        .size(13.0)
                                                        .color(muted_color)
                                                );
                                            });
                                        }
                                    }
                                });
                            });
                        }

                        // Error Handling
                        if let Some(error) = &self.error {
                            ui.add_space(8.0);
                            ui.colored_label(
                                egui::Color32::from_rgb(255, 59, 48),
                                error
                            );
                        }
                    });
                });
            });
        });
    }
}

async fn classify_image(api_key: &str, image_bytes: &[u8]) -> anyhow::Result<AvalancheAnalysis> {
    use base64::Engine;
    let image_base64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);
    
    let client = reqwest::Client::new();
    let response = client
    .post("https://api.openai.com/v1/chat/completions")
    .header("Authorization", format!("Bearer {}", api_key))
    .json(&serde_json::json!({
        "model": "gpt-4o-mini",
        "response_format": { "type": "json_object" },
        "messages": [{
            "role": "user",
            "content": [
                {"type": "text", "text": r#"Analyze this mountain terrain for avalanche characteristics with extreme detail. Return a JSON object with this structure:
{
    "avalanche_present": boolean,
    "avalanche_type": "powder"|"loose-snow"|"slab"|"none",
    "confidence_level": 0.0-100.0,
    "terrain_features": string[],
    "visual_characteristics": {
        "powder_cloud": boolean,
        "fracture_line": boolean,
        "fracture_depth": "shallow"|"deep"|"variable"|null,
        "point_release": boolean,
        "debris_pattern": "fan-shaped"|"linear"|"scattered"|"none",
        "snow_texture": {
            "granular": boolean,
            "blocky": boolean,
            "fluffy": boolean,
            "density": "low"|"medium"|"high"
        },
        "movement_pattern": {
            "starting_width": "point"|"wide"|"undefined",
            "propagation": "fan"|"linear"|"chaotic"|"none",
            "vertical_movement": boolean,
            "lateral_spread": boolean
        },
        "terrain": {
            "slope_angle": "steep (>45°)"|"moderate (30-45°)"|"gentle (<30°)"|null,
            "surface_roughness": "smooth"|"rough"|"variable",
            "anchoring_points": boolean,
            "convex_rollover": boolean
        }
    }
}

DETAILED ANALYSIS GUIDELINES:

1. Snow Texture Analysis:
   - Granular: Individual snow particles visible? Common in loose snow
   - Blocky: Cohesive blocks or chunks? Typical of slab
   - Fluffy: Light, airy appearance? Common in powder
   - Density: Assess snow compactness

2. Movement Pattern Analysis:
   - Starting Width: Point source vs wide initial fracture
   - Propagation: How the avalanche spreads
   - Vertical Movement: Significant up/down motion
   - Lateral Spread: Sideways expansion

3. Terrain Analysis:
   - Slope Angle: Critical for type determination
   - Surface Roughness: Affects release pattern
   - Anchoring Points: Trees/rocks that affect flow
   - Convex Rollover: Terrain shape at release point

AVALANCHE TYPE CHARACTERISTICS:

LOOSE-SNOW Avalanche:
PRIMARY Indicators:
- Starting_width: "point"
- Propagation: "fan"
- Snow_texture: granular=true, blocky=false
- Debris_pattern: "fan-shaped"
SECONDARY Indicators:
- No distinct fracture line
- Low to medium density
- Often on steeper slopes
- Minimal lateral spread

SLAB Avalanche:
PRIMARY Indicators:
- Fracture_line: true
- Snow_texture: blocky=true
- Starting_width: "wide"
- Propagation: "linear"
SECONDARY Indicators:
- Medium to high density
- Linear debris pattern
- Moderate slope angles
- Significant lateral spread

POWDER Avalanche:
PRIMARY Indicators:
- Powder_cloud: true
- Snow_texture: fluffy=true
- Vertical_movement: true
SECONDARY Indicators:
- Low density
- Significant vertical displacement
- Often on steep terrain
- Chaotic propagation

Analyze ALL characteristics before classification. If mixed indicators present, weight PRIMARY indicators more heavily. A single PRIMARY indicator is not enough - require multiple matching characteristics for classification."#},
                {"type": "image_url", "image_url": {
                    "url": format!("data:image/jpeg;base64,{}", image_base64),
                    "detail": "high"
                }}
            ]
        }],
        "max_tokens": 600
    }))
    .send()
    .await?;

    let response_text = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&response_text)?;
    
    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Empty API response"))?;

    let analysis: AvalancheAnalysis = serde_json::from_str(content)
        .map_err(|e| anyhow::anyhow!("JSON parse error: {}\nResponse: {}", e, content))?;

    // Validate and score the avalanche type based on detailed characteristics
    if analysis.avalanche_present {
        let chars = &analysis.visual_characteristics;
        let snow = &chars.snow_texture;
        let movement = &chars.movement_pattern;
        
        // Calculate characteristic scores for each type
        let powder_score = {
            let mut score = 0i32;
            if chars.powder_cloud { score += 3; }  // Primary
            if snow.fluffy { score += 3; }        // Primary
            if movement.vertical_movement { score += 3; }  // Primary
            if snow.density == "low" { score += 1; }      // Secondary
            if movement.propagation == "chaotic" { score += 1; }  // Secondary
            if chars.terrain.slope_angle.as_ref().map_or(false, |a| a.starts_with("steep")) { score += 1; }
            score
        };

        let loose_snow_score = {
            let mut score = 0i32;
            if movement.starting_width == "point" { score += 3; }  // Primary
            if movement.propagation == "fan" { score += 3; }      // Primary
            if snow.granular { score += 3; }                      // Primary
            if chars.debris_pattern == "fan-shaped" { score += 3; }  // Primary
            if !chars.fracture_line { score += 1; }               // Secondary
            if snow.density == "low" { score += 1; }              // Secondary
            if chars.terrain.slope_angle.as_ref().map_or(false, |a| a.starts_with("steep")) { score += 1; }
            score
        };

        let slab_score = {
            let mut score = 0i32;
            if chars.fracture_line { score += 3; }               // Primary
            if snow.blocky { score += 3; }                      // Primary
            if movement.starting_width == "wide" { score += 3; } // Primary
            if movement.propagation == "linear" { score += 3; }  // Primary
            if snow.density == "high" { score += 1; }           // Secondary
            if chars.debris_pattern == "linear" { score += 1; }  // Secondary
            if movement.lateral_spread { score += 1; }           // Secondary
            score
        };

        // Determine highest scoring type
        let detected_type = analysis.avalanche_type.as_str();
        let (highest_score, expected_type) = [
            (powder_score, "powder"),
            (loose_snow_score, "loose-snow"),
            (slab_score, "slab")
        ].iter()
        .max_by_key(|&&(score, _)| score)
        .copied()
        .unwrap();

        // Require a minimum score difference for classification
        let second_highest_score = [powder_score, loose_snow_score, slab_score]
            .iter()
            .filter(|&&score| score != highest_score)
            .max()
            .copied()
            .unwrap();

        // If scores are too close or score is too low, classification is unreliable
        if (highest_score - second_highest_score) < 3 {
            return Err(anyhow::anyhow!(
                "Classification uncertainty: Multiple types show similar characteristics"
            ));
        }

        if highest_score < 6 {
            return Err(anyhow::anyhow!(
                "Insufficient characteristic evidence for classification"
            ));
        }

        // Verify classification matches highest scoring type
        if detected_type != expected_type {
            return Err(anyhow::anyhow!(
                "Inconsistent classification: Visual characteristics strongly indicate {} (score: {}) but classified as {}", 
                expected_type, highest_score, detected_type
            ));
        }
    }

    if !["powder", "loose-snow", "slab", "none"].contains(&analysis.avalanche_type.as_str()) {
        return Err(anyhow::anyhow!(
            "Invalid avalanche type: {}",
            analysis.avalanche_type
        ));
    }

    if analysis.confidence_level < 0.0 || analysis.confidence_level > 100.0 {
        return Err(anyhow::anyhow!(
            "Invalid confidence level: {}",
            analysis.confidence_level
        ));
    }

    Ok(analysis)
}

// Helper function for consistent pill labels
fn pill_label(text: &str, color: egui::Color32) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        let padding = egui::vec2(6.0, 2.0);
        let text = egui::RichText::new(text)
            .color(egui::Color32::WHITE)
            .size(12.0);
        
        let frame = egui::Frame::none()
            .fill(color)
            .rounding(egui::Rounding::same(8.0))
            .inner_margin(padding);

        frame.show(ui, |ui| {
            ui.label(text)
        }).response
    }
}

// Helper function for consistent info rows
fn info_row(ui: &mut egui::Ui, label: &str, value: &str, color: egui::Color32, size: f32) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .size(size)
                .color(color)
        );
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(value)
                .size(size)
                .strong()
        );
    });
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 800.0])
            .with_close_button(false),
        ..Default::default()
    };

    eframe::run_native(
        "Avalanche Detection and Risk Analyzer",
        options,
        Box::new(|cc| Box::new(AvalancheClassifier::new(cc))),
    )
    .unwrap();
}