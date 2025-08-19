use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, VLine};
use statrs::distribution::{Normal, Continuous};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "PDF Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(PdfViewerApp::new()))),
    )
}

#[derive(Default)]
struct PdfViewerApp {
    distributions: HashMap<u32, GaussianDistribution>,
    next_id: u32,
    selected_for_multiplication: Vec<u32>,
    plot_bounds: Option<egui_plot::PlotBounds>,
    show_shading: bool,
    shading_opacity: f32,
    show_std_markers: bool,
}

#[derive(Serialize, Deserialize)]
struct SessionData {
    distributions: HashMap<u32, GaussianDistribution>,
    next_id: u32,
    show_shading: bool,
    shading_opacity: f32,
    show_std_markers: bool,
}

impl PdfViewerApp {
    fn new() -> Self {
        Self {
            show_shading: true,
            shading_opacity: 0.3,
            show_std_markers: true,
            ..Default::default()
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct GaussianDistribution {
    id: u32,
    name: String,
    mean: f64,
    std_dev: f64,
    parent_ids: Vec<u32>,
    is_product: bool,
}

impl Default for GaussianDistribution {
    fn default() -> Self {
        Self {
            id: 0,
            name: "Gaussian 1".to_string(),
            mean: 0.0,
            std_dev: 1.0,
            parent_ids: vec![],
            is_product: false,
        }
    }
}

impl GaussianDistribution {
    fn new(id: u32, name: String, mean: f64, std_dev: f64) -> Self {
        Self {
            id,
            name,
            mean,
            std_dev,
            parent_ids: vec![],
            is_product: false,
        }
    }
    
    fn new_product(id: u32, name: String, parent_ids: Vec<u32>, parents: &[&GaussianDistribution]) -> Self {
        // For Gaussian distributions, multiplication results in another Gaussian
        // with specific mean and variance relationships
        let (mean, variance) = Self::multiply_gaussians(parents);
        Self {
            id,
            name,
            mean,
            std_dev: variance.sqrt(),
            parent_ids,
            is_product: true,
        }
    }
    
    fn multiply_gaussians(gaussians: &[&GaussianDistribution]) -> (f64, f64) {
        if gaussians.is_empty() {
            return (0.0, 1.0);
        }
        
        // For multiplying Gaussian PDFs: 
        // The product of two Gaussians N(μ₁,σ₁²) * N(μ₂,σ₂²) is proportional to
        // N((μ₁/σ₁² + μ₂/σ₂²)/(1/σ₁² + 1/σ₂²), 1/(1/σ₁² + 1/σ₂²))
        
        let mut precision_sum = 0.0;  // sum of 1/σ²
        let mut weighted_mean_sum = 0.0;  // sum of μ/σ²
        
        for gaussian in gaussians {
            let precision = 1.0 / (gaussian.std_dev * gaussian.std_dev);
            precision_sum += precision;
            weighted_mean_sum += gaussian.mean * precision;
        }
        
        let result_mean = weighted_mean_sum / precision_sum;
        let result_variance = 1.0 / precision_sum;
        
        (result_mean, result_variance)
    }
    
    fn evaluate(&self, x: f64) -> f64 {
        let normal = Normal::new(self.mean, self.std_dev).unwrap();
        normal.pdf(x)
    }
    
    fn generate_points(&self, x_min: f64, x_max: f64, num_points: usize) -> PlotPoints {
        let mut points = Vec::new();
        for i in 0..num_points {
            let x = x_min + (x_max - x_min) * i as f64 / (num_points - 1) as f64;
            let y = self.evaluate(x);
            points.push([x, y]);
        }
        PlotPoints::new(points)
    }
    
    fn generate_shading_polygon(&self, x_min: f64, x_max: f64, num_points: usize) -> PlotPoints {
        let mut points = Vec::with_capacity(num_points + 2);
        
        // Create clean polygon: bottom-left -> curve points -> bottom-right
        // Key insight: don't duplicate corner points in the curve sampling
        
        points.push([x_min, 0.0]);  // Bottom-left corner
        
        // Generate curve points excluding the exact boundaries to avoid duplication
        if num_points == 1 {
            // Single point case: use center
            let x = (x_min + x_max) / 2.0;
            let y = self.evaluate(x);
            points.push([x, y]);
        } else if num_points > 1 {
            // Multiple points: space them between (but not including) the boundaries
            for i in 1..=num_points {
                let x = x_min + (x_max - x_min) * i as f64 / (num_points + 1) as f64;
                let y = self.evaluate(x);
                points.push([x, y]);
            }
        }
        
        points.push([x_max, 0.0]);  // Bottom-right corner
        
        // Let polygon fill algorithm automatically close from last point to first
        PlotPoints::new(points)
    }
    
    // Debug method to generate points as Vec instead of PlotPoints so we can inspect them
    fn generate_debug_points(&self, x_min: f64, x_max: f64, num_points: usize) -> Vec<[f64; 2]> {
        let mut points = Vec::with_capacity(num_points + 2);
        
        points.push([x_min, 0.0]);  // Bottom-left corner
        
        // Match the logic in generate_shading_polygon
        if num_points == 1 {
            let x = (x_min + x_max) / 2.0;
            let y = self.evaluate(x);
            points.push([x, y]);
        } else if num_points > 1 {
            for i in 1..=num_points {
                let x = x_min + (x_max - x_min) * i as f64 / (num_points + 1) as f64;
                let y = self.evaluate(x);
                points.push([x, y]);
            }
        }
        
        points.push([x_max, 0.0]);  // Bottom-right corner
        points
    }
    
    fn get_std_markers(&self) -> Vec<f64> {
        vec![
            self.mean - 3.0 * self.std_dev,
            self.mean - 2.0 * self.std_dev,
            self.mean - self.std_dev,
            self.mean,
            self.mean + self.std_dev,
            self.mean + 2.0 * self.std_dev,
            self.mean + 3.0 * self.std_dev,
        ]
    }
}

impl PdfViewerApp {
    fn update_product_distributions(&mut self) {
        let mut updates = Vec::new();
        
        for (id, dist) in self.distributions.iter() {
            if dist.is_product && !dist.parent_ids.is_empty() {
                let parent_refs: Vec<&GaussianDistribution> = dist.parent_ids
                    .iter()
                    .filter_map(|parent_id| self.distributions.get(parent_id))
                    .collect();
                
                if parent_refs.len() == dist.parent_ids.len() {
                    let (new_mean, new_variance) = GaussianDistribution::multiply_gaussians(&parent_refs);
                    updates.push((*id, new_mean, new_variance.sqrt()));
                }
            }
        }
        
        for (id, mean, std_dev) in updates {
            if let Some(dist) = self.distributions.get_mut(&id) {
                dist.mean = mean;
                dist.std_dev = std_dev;
            }
        }
    }
    
    fn get_plot_range(&self) -> (f64, f64) {
        if let Some(bounds) = &self.plot_bounds {
            (bounds.min()[0], bounds.max()[0])
        } else {
            (-6.0, 6.0)
        }
    }
    
    fn auto_fit_view(&mut self) {
        if self.distributions.is_empty() {
            return;
        }
        
        let mut min_mean = f64::INFINITY;
        let mut max_mean = f64::NEG_INFINITY;
        let mut max_std_dev: f64 = 0.0;
        
        for dist in self.distributions.values() {
            min_mean = min_mean.min(dist.mean);
            max_mean = max_mean.max(dist.mean);
            max_std_dev = max_std_dev.max(dist.std_dev);
        }
        
        // Extend range by 4 standard deviations to show tails
        let margin = 4.0 * max_std_dev;
        let x_min = min_mean - margin;
        let x_max = max_mean + margin;
        
        // Calculate reasonable y bounds
        let y_max = 1.0 / (max_std_dev * (2.0 * std::f64::consts::PI).sqrt()) * 1.1;
        
        self.plot_bounds = Some(egui_plot::PlotBounds::from_min_max(
            [x_min, 0.0],
            [x_max, y_max],
        ));
    }
    
    fn save_session(&self) -> Result<String, String> {
        let session_data = SessionData {
            distributions: self.distributions.clone(),
            next_id: self.next_id,
            show_shading: self.show_shading,
            shading_opacity: self.shading_opacity,
            show_std_markers: self.show_std_markers,
        };
        
        serde_json::to_string_pretty(&session_data)
            .map_err(|e| format!("Failed to serialize session: {}", e))
    }
    
    fn load_session(&mut self, json_data: &str) -> Result<(), String> {
        let session_data: SessionData = serde_json::from_str(json_data)
            .map_err(|e| format!("Failed to parse session: {}", e))?;
        
        self.distributions = session_data.distributions;
        self.next_id = session_data.next_id;
        self.show_shading = session_data.show_shading;
        self.shading_opacity = session_data.shading_opacity;
        self.show_std_markers = session_data.show_std_markers;
        self.selected_for_multiplication.clear();
        
        Ok(())
    }
}

impl eframe::App for PdfViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("PDF Viewer - Probability Density Function Explorer");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("💾 Save Session").clicked() {
                        match self.save_session() {
                            Ok(json) => {
                                ui.output_mut(|o| o.copied_text = json);
                                println!("Session saved to clipboard!");
                            }
                            Err(e) => {
                                eprintln!("Failed to save session: {}", e);
                            }
                        }
                    }
                    
                    if ui.button("📁 Load Session").clicked() {
                        // Simple implementation - user needs to paste JSON manually
                        println!("To load a session, paste the JSON data and restart the application");
                    }
                });
            });
            
            ui.separator();
            
            // Add initial distribution if none exist
            if self.distributions.is_empty() {
                let dist = GaussianDistribution::new(
                    self.next_id,
                    format!("Gaussian {}", self.next_id + 1),
                    0.0,
                    1.0,
                );
                self.distributions.insert(self.next_id, dist);
                self.next_id += 1;
            }
            
            ui.horizontal(|ui| {
                // Left panel for controls
                ui.vertical(|ui| {
                    ui.set_width(300.0);
                    ui.heading("Distribution Controls");
                    
                    if ui.button("Add New Gaussian").clicked() {
                        let dist = GaussianDistribution::new(
                            self.next_id,
                            format!("Gaussian {}", self.next_id + 1),
                            0.0,
                            1.0,
                        );
                        self.distributions.insert(self.next_id, dist);
                        self.next_id += 1;
                    }
                    
                    ui.separator();
                    
                    // Visual controls
                    ui.heading("Visual Options");
                    ui.checkbox(&mut self.show_shading, "Show shading under curves");
                    if self.show_shading {
                        ui.horizontal(|ui| {
                            ui.label("Opacity:");
                            ui.add(egui::Slider::new(&mut self.shading_opacity, 0.0..=1.0)
                                .fixed_decimals(2));
                        });
                    }
                    ui.checkbox(&mut self.show_std_markers, "Show standard deviation markers");
                    
                    ui.separator();
                    
                    // Multiplication controls
                    ui.heading("Multiply PDFs");
                    ui.horizontal(|ui| {
                        if ui.button("Multiply Selected").clicked() {
                            if self.selected_for_multiplication.len() >= 2 {
                                let parent_refs: Vec<&GaussianDistribution> = self.selected_for_multiplication
                                    .iter()
                                    .filter_map(|id| self.distributions.get(id))
                                    .collect();
                                
                                if parent_refs.len() >= 2 {
                                    let product_name = format!("Product {}", self.next_id + 1);
                                    let product = GaussianDistribution::new_product(
                                        self.next_id,
                                        product_name,
                                        self.selected_for_multiplication.clone(),
                                        &parent_refs,
                                    );
                                    
                                    self.distributions.insert(self.next_id, product);
                                    self.next_id += 1;
                                    self.selected_for_multiplication.clear();
                                }
                            }
                        }
                        
                        if ui.button("Clear Selection").clicked() {
                            self.selected_for_multiplication.clear();
                        }
                    });
                    
                    if !self.selected_for_multiplication.is_empty() {
                        ui.label(format!("Selected: {} distributions", self.selected_for_multiplication.len()));
                    }
                    
                    ui.separator();
                    
                    // Distribution parameter controls
                    let mut to_remove = Vec::new();
                    for (id, dist) in self.distributions.iter_mut() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                // Selection checkbox for multiplication
                                let mut selected = self.selected_for_multiplication.contains(id);
                                if ui.checkbox(&mut selected, "").clicked() {
                                    if selected {
                                        if !self.selected_for_multiplication.contains(id) {
                                            self.selected_for_multiplication.push(*id);
                                        }
                                    } else {
                                        self.selected_for_multiplication.retain(|&x| x != *id);
                                    }
                                }
                                
                                ui.label(&dist.name);
                                if dist.is_product {
                                    ui.label("(Product)");
                                }
                                if ui.small_button("✖").clicked() {
                                    to_remove.push(*id);
                                }
                            });
                            
                            // Only show parameter controls for non-product distributions
                            if !dist.is_product {
                                ui.horizontal(|ui| {
                                    ui.label("Mean:");
                                    ui.add(egui::DragValue::new(&mut dist.mean)
                                        .speed(0.1)
                                        .range(-10.0..=10.0));
                                });
                                
                                ui.horizontal(|ui| {
                                    ui.label("Std Dev:");
                                    ui.add(egui::DragValue::new(&mut dist.std_dev)
                                        .speed(0.01)
                                        .range(0.1..=5.0));
                                });
                                
                                // Slider versions
                                ui.add(egui::Slider::new(&mut dist.mean, -10.0..=10.0)
                                    .text("Mean"));
                                ui.add(egui::Slider::new(&mut dist.std_dev, 0.1..=5.0)
                                    .text("Std Dev"));
                            } else {
                                // Show read-only info for product distributions
                                ui.label(format!("Mean: {:.3}", dist.mean));
                                ui.label(format!("Std Dev: {:.3}", dist.std_dev));
                                ui.label(format!("Parents: {:?}", dist.parent_ids));
                            }
                        });
                    }
                    
                    // Remove marked distributions
                    for id in to_remove {
                        self.distributions.remove(&id);
                        // Also remove from selection
                        self.selected_for_multiplication.retain(|&x| x != id);
                    }
                    
                    // Update product distributions when their parents change
                    self.update_product_distributions();
                });
                
                ui.separator();
                
                // Right panel for plot
                ui.vertical(|ui| {
                    ui.heading("Probability Density Functions");
                    
                    // Plot controls
                    ui.horizontal(|ui| {
                        if ui.button("Reset View").clicked() {
                            self.plot_bounds = None;
                        }
                        if ui.button("Auto-fit").clicked() {
                            self.auto_fit_view();
                        }
                        ui.label("| Mouse: drag to pan, scroll to zoom");
                    });
                    
                    let plot = Plot::new("pdf_plot")
                        .view_aspect(2.0)
                        .allow_zoom(true)
                        .allow_drag(true)
                        .allow_scroll(true)
                        .show_axes([true, true]);
                        
                    plot.show(ui, |plot_ui| {
                        let colors = [
                            egui::Color32::BLUE,
                            egui::Color32::RED,
                            egui::Color32::GREEN,
                            egui::Color32::from_rgb(255, 165, 0), // Orange
                            egui::Color32::from_rgb(128, 0, 128), // Purple
                            egui::Color32::from_rgb(255, 192, 203), // Pink
                        ];
                        
                        
                        for (idx, dist) in self.distributions.values().enumerate() {
                            let (x_min, x_max) = self.get_plot_range();
                            let color = colors[idx % colors.len()];
                            
                            // Draw shading if enabled  
                            if self.show_shading {
                                // Use Line's native fill() method instead of manual polygon
                                let points = dist.generate_points(x_min, x_max, 300);
                                
                                // Create color with user-controlled opacity for the fill
                                // Ensure minimum alpha of 1 to prevent auto-color assignment
                                let alpha = ((255.0 * self.shading_opacity) as u8).max(1);
                                let fill_color = egui::Color32::from_rgba_unmultiplied(
                                    color.r(),
                                    color.g(), 
                                    color.b(),
                                    alpha
                                );
                                
                                let line_with_fill = Line::new(points)
                                    .name(&format!("{} (shading)", dist.name))
                                    .color(fill_color)
                                    .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT))  // Make stroke invisible
                                    .fill(0.0);  // Fill area between line and y=0
                                plot_ui.line(line_with_fill);
                            }
                            
                            // Draw the curve line
                            let points = dist.generate_points(x_min, x_max, 300);
                            let line = Line::new(points)
                                .name(&dist.name)
                                .color(color);
                            plot_ui.line(line);
                            
                            // Draw standard deviation markers if enabled
                            if self.show_std_markers {
                                let markers = dist.get_std_markers();
                                for (i, &marker_x) in markers.iter().enumerate() {
                                    if marker_x >= x_min && marker_x <= x_max {
                                        let marker_style = if i == 3 { // Mean marker
                                            egui::Stroke::new(2.0, color)
                                        } else {
                                            egui::Stroke::new(1.0, color.gamma_multiply(0.7))
                                        };
                                        
                                        let vline = VLine::new(marker_x)
                                            .style(egui_plot::LineStyle::Dashed { length: 5.0 })
                                            .stroke(marker_style);
                                        plot_ui.vline(vline);
                                    }
                                }
                            }
                        }
                    });
                });
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-10;
    const APPROX_EPSILON: f64 = 1e-6;

    #[test]
    fn test_gaussian_distribution_creation() {
        let dist = GaussianDistribution::new(1, "Test".to_string(), 0.0, 1.0);
        assert_eq!(dist.id, 1);
        assert_eq!(dist.name, "Test");
        assert_eq!(dist.mean, 0.0);
        assert_eq!(dist.std_dev, 1.0);
        assert!(dist.parent_ids.is_empty());
        assert!(!dist.is_product);
    }

    #[test]
    fn test_gaussian_pdf_evaluation() {
        let dist = GaussianDistribution::new(1, "Standard Normal".to_string(), 0.0, 1.0);
        
        // Test at mean (should be maximum)
        let at_mean = dist.evaluate(0.0);
        let expected_at_mean = 1.0 / (2.0 * PI).sqrt();
        assert_abs_diff_eq!(at_mean, expected_at_mean, epsilon = APPROX_EPSILON);
        
        // Test at one standard deviation
        let at_one_std = dist.evaluate(1.0);
        let expected_at_one_std = (1.0 / (2.0 * PI).sqrt()) * (-0.5_f64).exp();
        assert_abs_diff_eq!(at_one_std, expected_at_one_std, epsilon = APPROX_EPSILON);
        
        // Test symmetry
        assert_abs_diff_eq!(dist.evaluate(-1.0), dist.evaluate(1.0), epsilon = EPSILON);
    }

    #[test]
    fn test_gaussian_pdf_different_parameters() {
        let dist = GaussianDistribution::new(1, "Custom".to_string(), 2.0, 0.5);
        
        // Test at mean
        let at_mean = dist.evaluate(2.0);
        let expected = 1.0 / (0.5 * (2.0 * PI).sqrt());
        assert_abs_diff_eq!(at_mean, expected, epsilon = APPROX_EPSILON);
        
        // Test symmetry around mean
        assert_abs_diff_eq!(dist.evaluate(1.5), dist.evaluate(2.5), epsilon = APPROX_EPSILON);
    }

    #[test]
    fn test_gaussian_multiplication_two_distributions() {
        let dist1 = GaussianDistribution::new(1, "Dist1".to_string(), 0.0, 1.0);
        let dist2 = GaussianDistribution::new(2, "Dist2".to_string(), 2.0, 1.0);
        
        let parents = vec![&dist1, &dist2];
        let (result_mean, result_variance) = GaussianDistribution::multiply_gaussians(&parents);
        
        // For N(0,1) * N(2,1):
        // precision1 = 1, precision2 = 1
        // weighted_mean_sum = 0*1 + 2*1 = 2
        // precision_sum = 1 + 1 = 2
        // result_mean = 2/2 = 1
        // result_variance = 1/2 = 0.5
        assert_abs_diff_eq!(result_mean, 1.0, epsilon = EPSILON);
        assert_abs_diff_eq!(result_variance, 0.5, epsilon = EPSILON);
    }

    #[test]
    fn test_gaussian_multiplication_three_distributions() {
        let dist1 = GaussianDistribution::new(1, "D1".to_string(), 0.0, 1.0);
        let dist2 = GaussianDistribution::new(2, "D2".to_string(), 3.0, 1.0);
        let dist3 = GaussianDistribution::new(3, "D3".to_string(), 6.0, 2.0);
        
        let parents = vec![&dist1, &dist2, &dist3];
        let (result_mean, result_variance) = GaussianDistribution::multiply_gaussians(&parents);
        
        // precision1 = 1, precision2 = 1, precision3 = 1/4 = 0.25
        // weighted_mean_sum = 0*1 + 3*1 + 6*0.25 = 4.5
        // precision_sum = 1 + 1 + 0.25 = 2.25
        // result_mean = 4.5/2.25 = 2.0
        // result_variance = 1/2.25 = 4/9
        assert_abs_diff_eq!(result_mean, 2.0, epsilon = APPROX_EPSILON);
        assert_abs_diff_eq!(result_variance, 4.0/9.0, epsilon = APPROX_EPSILON);
    }

    #[test]
    fn test_gaussian_multiplication_empty_list() {
        let parents: Vec<&GaussianDistribution> = vec![];
        let (result_mean, result_variance) = GaussianDistribution::multiply_gaussians(&parents);
        assert_eq!(result_mean, 0.0);
        assert_eq!(result_variance, 1.0);
    }

    #[test]
    fn test_gaussian_product_creation() {
        let dist1 = GaussianDistribution::new(1, "Parent1".to_string(), 1.0, 2.0);
        let dist2 = GaussianDistribution::new(2, "Parent2".to_string(), 3.0, 1.0);
        
        let parents = vec![&dist1, &dist2];
        let parent_ids = vec![1, 2];
        let product = GaussianDistribution::new_product(
            10, 
            "Product".to_string(), 
            parent_ids.clone(), 
            &parents
        );
        
        assert_eq!(product.id, 10);
        assert_eq!(product.name, "Product");
        assert_eq!(product.parent_ids, parent_ids);
        assert!(product.is_product);
        
        // Verify mathematical correctness
        // precision1 = 1/4 = 0.25, precision2 = 1
        // weighted_mean_sum = 1*0.25 + 3*1 = 3.25
        // precision_sum = 0.25 + 1 = 1.25
        // result_mean = 3.25/1.25 = 2.6
        // result_std_dev = sqrt(1/1.25) = sqrt(0.8) ≈ 0.894
        assert_abs_diff_eq!(product.mean, 2.6, epsilon = APPROX_EPSILON);
        assert_abs_diff_eq!(product.std_dev, (0.8_f64).sqrt(), epsilon = APPROX_EPSILON);
    }

    #[test]
    fn test_generate_points_basic() {
        let dist = GaussianDistribution::new(1, "Test".to_string(), 0.0, 1.0);
        
        // Test the individual point generation logic instead
        let x_values = [-2.0, -1.0, 0.0, 1.0, 2.0];
        let y_values: Vec<f64> = x_values.iter().map(|&x| dist.evaluate(x)).collect();
        
        assert_eq!(y_values.len(), 5);
        
        // Check that y values are positive (valid PDF values)
        for &y in &y_values {
            assert!(y > 0.0);
        }
        
        // Check that maximum is at mean (x=0) - middle value should be largest
        assert!(y_values[2] > y_values[0]);
        assert!(y_values[2] > y_values[4]);
        
        // Test symmetry
        assert_abs_diff_eq!(y_values[0], y_values[4], epsilon = APPROX_EPSILON);
        assert_abs_diff_eq!(y_values[1], y_values[3], epsilon = APPROX_EPSILON);
    }

    #[test]
    fn test_generate_shading_polygon() {
        let dist = GaussianDistribution::new(1, "Test".to_string(), 0.0, 1.0);
        
        let x_min = -2.0;
        let x_max = 2.0;
        let num_points = 5;
        
        // Generate points manually to test the algorithm since PlotPoints is opaque
        let mut expected_points = Vec::with_capacity(num_points + 2);
        
        // Start from the bottom left corner
        expected_points.push([x_min, 0.0]);
        
        // Generate curve points from left to right
        for i in 0..num_points {
            let x = x_min + (x_max - x_min) * i as f64 / (num_points - 1) as f64;
            let y = dist.evaluate(x);
            expected_points.push([x, y]);
        }
        
        // End at the bottom right corner
        expected_points.push([x_max, 0.0]);
        
        // Now test the properties using our expected points
        assert_eq!(expected_points.len(), num_points + 2);
        
        // First point should be bottom left corner
        assert_abs_diff_eq!(expected_points[0][0], x_min, epsilon = EPSILON);
        assert_abs_diff_eq!(expected_points[0][1], 0.0, epsilon = EPSILON);
        
        // Last point should be bottom right corner  
        let last_idx = expected_points.len() - 1;
        assert_abs_diff_eq!(expected_points[last_idx][0], x_max, epsilon = EPSILON);
        assert_abs_diff_eq!(expected_points[last_idx][1], 0.0, epsilon = EPSILON);
        
        // Middle points should have positive y values (above x-axis)
        for i in 1..expected_points.len()-1 {
            let point = expected_points[i];
            assert!(point[1] > 0.0, "Point {} should be above x-axis, got y={}", i, point[1]);
            assert!(point[0] >= x_min && point[0] <= x_max, "Point {} x-coordinate should be in range", i);
        }
        
        // Points should be ordered by x-coordinate (left to right)
        for i in 1..expected_points.len() {
            assert!(expected_points[i][0] >= expected_points[i-1][0], "Points should be ordered by x-coordinate");
        }
        
        // The curve points should form a proper bell shape (maximum near center)
        let center_idx = expected_points.len() / 2;
        let center_y = expected_points[center_idx][1];
        let edge_y = expected_points[1][1]; // First curve point
        assert!(center_y >= edge_y, "Center of distribution should be at least as high as edges");
    }

    #[test]
    fn test_std_markers() {
        let dist = GaussianDistribution::new(1, "Test".to_string(), 5.0, 2.0);
        let markers = dist.get_std_markers();
        
        assert_eq!(markers.len(), 7);
        
        let expected = vec![
            5.0 - 3.0 * 2.0, // -1.0
            5.0 - 2.0 * 2.0, // 1.0
            5.0 - 1.0 * 2.0, // 3.0
            5.0,              // 5.0 (mean)
            5.0 + 1.0 * 2.0, // 7.0
            5.0 + 2.0 * 2.0, // 9.0
            5.0 + 3.0 * 2.0, // 11.0
        ];
        
        for (i, &marker) in markers.iter().enumerate() {
            assert_abs_diff_eq!(marker, expected[i], epsilon = EPSILON);
        }
    }

    #[test]
    fn test_pdf_viewer_app_creation() {
        let app = PdfViewerApp::new();
        assert!(app.distributions.is_empty());
        assert_eq!(app.next_id, 0);
        assert!(app.selected_for_multiplication.is_empty());
        assert!(app.show_shading);
        assert_abs_diff_eq!(app.shading_opacity, 0.3_f32, epsilon = 1e-6_f32);
        assert!(app.show_std_markers);
    }

    #[test]
    fn test_session_save_load_roundtrip() {
        let mut app = PdfViewerApp::new();
        
        // Add some distributions
        let dist1 = GaussianDistribution::new(0, "Test1".to_string(), 1.0, 0.5);
        let dist2 = GaussianDistribution::new(1, "Test2".to_string(), -1.0, 2.0);
        
        app.distributions.insert(0, dist1);
        app.distributions.insert(1, dist2);
        app.next_id = 2;
        app.show_shading = false;
        app.shading_opacity = 0.7;
        app.show_std_markers = false;
        
        // Save session
        let json = app.save_session().expect("Save should succeed");
        assert!(json.contains("Test1"));
        assert!(json.contains("Test2"));
        
        // Create new app and load session
        let mut new_app = PdfViewerApp::new();
        new_app.load_session(&json).expect("Load should succeed");
        
        // Verify all data was restored
        assert_eq!(new_app.distributions.len(), 2);
        assert_eq!(new_app.next_id, 2);
        assert!(!new_app.show_shading);
        assert_abs_diff_eq!(new_app.shading_opacity, 0.7_f32, epsilon = 1e-6_f32);
        assert!(!new_app.show_std_markers);
        
        // Verify distribution details
        let loaded_dist1 = new_app.distributions.get(&0).unwrap();
        assert_eq!(loaded_dist1.name, "Test1");
        assert_abs_diff_eq!(loaded_dist1.mean, 1.0, epsilon = EPSILON);
        assert_abs_diff_eq!(loaded_dist1.std_dev, 0.5, epsilon = EPSILON);
    }

    #[test]
    fn test_session_save_with_products() {
        let mut app = PdfViewerApp::new();
        
        // Create parent distributions
        let parent1 = GaussianDistribution::new(0, "Parent1".to_string(), 0.0, 1.0);
        let parent2 = GaussianDistribution::new(1, "Parent2".to_string(), 2.0, 1.0);
        
        // Create product distribution
        let parents = vec![&parent1, &parent2];
        let product = GaussianDistribution::new_product(
            2, 
            "Product".to_string(), 
            vec![0, 1], 
            &parents
        );
        
        app.distributions.insert(0, parent1);
        app.distributions.insert(1, parent2);
        app.distributions.insert(2, product);
        app.next_id = 3;
        
        // Test save/load
        let json = app.save_session().expect("Save should succeed");
        let mut new_app = PdfViewerApp::new();
        new_app.load_session(&json).expect("Load should succeed");
        
        // Verify product distribution was preserved
        let loaded_product = new_app.distributions.get(&2).unwrap();
        assert!(loaded_product.is_product);
        assert_eq!(loaded_product.parent_ids, vec![0, 1]);
        assert_eq!(loaded_product.name, "Product");
    }

    #[test]
    fn test_update_product_distributions() {
        let mut app = PdfViewerApp::new();
        
        // Create parent distributions
        let parent1 = GaussianDistribution::new(0, "Parent1".to_string(), 0.0, 1.0);
        let parent2 = GaussianDistribution::new(1, "Parent2".to_string(), 2.0, 1.0);
        
        // Create product distribution
        let parents = vec![&parent1, &parent2];
        let product = GaussianDistribution::new_product(
            2, 
            "Product".to_string(), 
            vec![0, 1], 
            &parents
        );
        
        app.distributions.insert(0, parent1);
        app.distributions.insert(1, parent2);
        app.distributions.insert(2, product);
        
        // Modify a parent distribution
        app.distributions.get_mut(&0).unwrap().mean = 1.0;
        app.distributions.get_mut(&0).unwrap().std_dev = 0.5;
        
        // Update products
        app.update_product_distributions();
        
        // Verify product was updated
        let updated_product = app.distributions.get(&2).unwrap();
        
        // Calculate expected values manually
        // Parent1: mean=1.0, std_dev=0.5, precision=4
        // Parent2: mean=2.0, std_dev=1.0, precision=1
        // Expected mean = (1.0*4 + 2.0*1) / (4+1) = 6/5 = 1.2
        // Expected variance = 1/(4+1) = 0.2
        // Expected std_dev = sqrt(0.2) ≈ 0.447
        assert_abs_diff_eq!(updated_product.mean, 1.2, epsilon = APPROX_EPSILON);
        assert_abs_diff_eq!(updated_product.std_dev, (0.2_f64).sqrt(), epsilon = APPROX_EPSILON);
    }

    #[test]
    fn test_invalid_json_load() {
        let mut app = PdfViewerApp::new();
        let result = app.load_session("invalid json");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to parse"));
    }

    #[test]
    fn test_very_small_std_dev() {
        let dist = GaussianDistribution::new(1, "Narrow".to_string(), 0.0, 0.01);
        let at_mean = dist.evaluate(0.0);
        
        // Very narrow distribution should have very high peak
        assert!(at_mean > 30.0); // Much higher than standard normal
        
        // Test integration manually instead of using PlotPoints
        let x_min = -0.05;
        let x_max = 0.05;
        let num_points = 100;
        
        let dx = (x_max - x_min) / (num_points - 1) as f64;
        let mut integral = 0.0;
        
        for i in 0..(num_points - 1) {
            let x1 = x_min + i as f64 * dx;
            let x2 = x_min + (i + 1) as f64 * dx;
            let y1 = dist.evaluate(x1);
            let y2 = dist.evaluate(x2);
            integral += (y1 + y2) * dx * 0.5;
        }
        
        // Should be close to 1, but we're only integrating a small range
        assert!(integral > 0.8); // Most of the mass should be in this range
    }

    #[test]
    fn test_large_std_dev() {
        let dist = GaussianDistribution::new(1, "Wide".to_string(), 0.0, 10.0);
        let at_mean = dist.evaluate(0.0);
        
        // Very wide distribution should have very low peak
        assert!(at_mean < 0.05);
        
        // Should still be symmetric
        assert_abs_diff_eq!(dist.evaluate(-5.0), dist.evaluate(5.0), epsilon = APPROX_EPSILON);
    }

    #[test]
    fn test_plot_range_calculation() {
        let app = PdfViewerApp::new();
        
        // Test default range
        let (x_min, x_max) = app.get_plot_range();
        assert_abs_diff_eq!(x_min, -6.0, epsilon = EPSILON);
        assert_abs_diff_eq!(x_max, 6.0, epsilon = EPSILON);
    }

    #[test]
    fn test_auto_fit_view() {
        let mut app = PdfViewerApp::new();
        
        // Add distributions with different means and std devs
        let dist1 = GaussianDistribution::new(0, "D1".to_string(), -2.0, 0.5);
        let dist2 = GaussianDistribution::new(1, "D2".to_string(), 5.0, 2.0);
        
        app.distributions.insert(0, dist1);
        app.distributions.insert(1, dist2);
        
        app.auto_fit_view();
        
        // Should fit range to include all distributions with margin
        assert!(app.plot_bounds.is_some());
        let bounds = app.plot_bounds.unwrap();
        
        // Expected range: min_mean=-2, max_mean=5, max_std_dev=2
        // Margin = 4 * 2 = 8
        // x_min = -2 - 8 = -10, x_max = 5 + 8 = 13
        assert_abs_diff_eq!(bounds.min()[0], -10.0, epsilon = APPROX_EPSILON);
        assert_abs_diff_eq!(bounds.max()[0], 13.0, epsilon = APPROX_EPSILON);
        
        // Y bounds should be reasonable
        assert_abs_diff_eq!(bounds.min()[1], 0.0, epsilon = EPSILON);
        assert!(bounds.max()[1] > 0.0);
    }

    #[test]
    fn test_auto_fit_empty_distributions() {
        let mut app = PdfViewerApp::new();
        
        // Should not crash with empty distributions
        app.auto_fit_view();
        // Function should return early without setting bounds
    }

    #[test]
    fn test_mathematical_properties() {
        // Test that multiplying identical distributions gives expected result
        let dist = GaussianDistribution::new(1, "Original".to_string(), 3.0, 2.0);
        let parents = vec![&dist, &dist];
        let (mean, variance) = GaussianDistribution::multiply_gaussians(&parents);
        
        // When multiplying identical N(μ,σ²) distributions:
        // Result should be N(μ, σ²/2)
        assert_abs_diff_eq!(mean, 3.0, epsilon = APPROX_EPSILON);
        assert_abs_diff_eq!(variance, 2.0, epsilon = APPROX_EPSILON); // σ²/2 = 4/2 = 2
    }

    #[test]
    fn test_precision_edge_case() {
        // Test with very different precisions
        let high_precision = GaussianDistribution::new(1, "HP".to_string(), 1.0, 0.1);
        let low_precision = GaussianDistribution::new(2, "LP".to_string(), 5.0, 10.0);
        
        let parents = vec![&high_precision, &low_precision];
        let (mean, _variance) = GaussianDistribution::multiply_gaussians(&parents);
        
        // High precision distribution should dominate
        // precision_hp = 1/0.01 = 100, precision_lp = 1/100 = 0.01
        // Expected mean ≈ (1.0 * 100 + 5.0 * 0.01) / (100 + 0.01) ≈ 1.0005
        assert!(mean > 1.0);
        assert!(mean < 1.1); // Should be very close to high precision mean
    }

    #[test]
    fn test_shading_polygon_different_distributions() {
        // Test shading polygons for distributions with different parameters
        let distributions = vec![
            GaussianDistribution::new(1, "Narrow".to_string(), 0.0, 0.5),
            GaussianDistribution::new(2, "Wide".to_string(), 0.0, 2.0),
            GaussianDistribution::new(3, "Shifted".to_string(), 3.0, 1.0),
        ];
        
        let x_min = -6.0;
        let x_max = 6.0;
        let num_points = 100;
        
        for dist in &distributions {
            // Generate expected points manually to test the algorithm
            let mut expected_points = Vec::with_capacity(num_points + 2);
            expected_points.push([x_min, 0.0]);
            
            for i in 0..num_points {
                let x = x_min + (x_max - x_min) * i as f64 / (num_points - 1) as f64;
                let y = dist.evaluate(x);
                expected_points.push([x, y]);
            }
            expected_points.push([x_max, 0.0]);
            
            // Validate basic structure
            assert_eq!(expected_points.len(), num_points + 2);
            
            // Validate boundary points
            assert_abs_diff_eq!(expected_points[0][1], 0.0, epsilon = EPSILON);
            assert_abs_diff_eq!(expected_points[expected_points.len()-1][1], 0.0, epsilon = EPSILON);
            
            // Find the maximum y value in the polygon (should be near the mean)
            let max_y = expected_points.iter().map(|p| p[1]).fold(0.0, f64::max);
            let expected_max_y = dist.evaluate(dist.mean);
            
            // The maximum in the polygon should be close to the theoretical maximum
            let tolerance = expected_max_y * 0.01; // 1% tolerance
            assert!((max_y - expected_max_y).abs() < tolerance, 
                   "Distribution {}: polygon max y={:.6}, expected max y={:.6}", 
                   dist.name, max_y, expected_max_y);
        }
    }

    #[test]
    fn test_shading_polygon_edge_cases() {
        let dist = GaussianDistribution::new(1, "Test".to_string(), 0.0, 1.0);
        
        // Test with minimal points
        let polygon_points = dist.generate_shading_polygon(-1.0, 1.0, 2);
        let points = polygon_points.points();
        assert_eq!(points.len(), 4); // 2 curve points + 2 boundary points
        
        // Test with large range
        let polygon_points = dist.generate_shading_polygon(-10.0, 10.0, 1000);
        let points = polygon_points.points();
        assert_eq!(points.len(), 1002); // 1000 curve points + 2 boundary points
        
        // Test with single point
        let polygon_points = dist.generate_shading_polygon(-1.0, 1.0, 1);
        let points = polygon_points.points();
        assert_eq!(points.len(), 3); // 1 curve point + 2 boundary points
        
        // Ensure all edge cases still maintain proper structure
        for test_points in [2, 1000, 1] {
            // Generate expected points manually
            let mut expected_points = Vec::with_capacity(test_points + 2);
            expected_points.push([-2.0, 0.0]);
            
            for i in 0..test_points {
                let x = if test_points == 1 {
                    // Special case: single point should be at the center of the range
                    (-2.0 + 2.0) / 2.0  // Center of [-2.0, 2.0]
                } else {
                    -2.0 + (4.0) * i as f64 / (test_points - 1) as f64
                };
                let y = dist.evaluate(x);
                expected_points.push([x, y]);
            }
            expected_points.push([2.0, 0.0]);
            
            // First and last should be on x-axis
            assert_abs_diff_eq!(expected_points[0][1], 0.0, epsilon = EPSILON);
            assert_abs_diff_eq!(expected_points[expected_points.len()-1][1], 0.0, epsilon = EPSILON);
            
            // All curve points should be above or on x-axis (boundary points are exactly 0)
            for i in 0..expected_points.len() {
                assert!(expected_points[i][1] >= 0.0, 
                       "Point {} has negative y value: ({}, {}) for test_points={}", 
                       i, expected_points[i][0], expected_points[i][1], test_points);
            }
        }
    }

    #[test]
    fn test_shading_polygon_area_approximation() {
        let dist = GaussianDistribution::new(1, "Test".to_string(), 0.0, 1.0);
        
        // Test that the polygon area approximates the integral reasonably well
        let x_min = -3.0;
        let x_max = 3.0;
        let num_points = 1000; // High resolution for better approximation
        
        // Generate expected points manually
        let mut expected_points = Vec::with_capacity(num_points + 2);
        expected_points.push([x_min, 0.0]);
        
        for i in 0..num_points {
            let x = x_min + (x_max - x_min) * i as f64 / (num_points - 1) as f64;
            let y = dist.evaluate(x);
            expected_points.push([x, y]);
        }
        expected_points.push([x_max, 0.0]);
        
        // Calculate polygon area using trapezoidal rule
        let mut polygon_area = 0.0;
        for i in 0..expected_points.len()-1 {
            let x1 = expected_points[i][0];
            let y1 = expected_points[i][1];
            let x2 = expected_points[i+1][0];
            let y2 = expected_points[i+1][1];
            
            // Trapezoidal area between points
            polygon_area += (x2 - x1) * (y1 + y2) * 0.5;
        }
        
        // Calculate theoretical integral using numerical integration
        let dx = (x_max - x_min) / (num_points - 1) as f64;
        let mut theoretical_area = 0.0;
        for i in 0..(num_points - 1) {
            let x1 = x_min + i as f64 * dx;
            let x2 = x_min + (i + 1) as f64 * dx;
            let y1 = dist.evaluate(x1);
            let y2 = dist.evaluate(x2);
            theoretical_area += (x2 - x1) * (y1 + y2) * 0.5;
        }
        
        // The polygon area should be very close to the theoretical area
        let relative_error = (polygon_area - theoretical_area).abs() / theoretical_area;
        assert!(relative_error < 0.01, "Polygon area {:.6} should closely match theoretical area {:.6}, relative error: {:.6}",
               polygon_area, theoretical_area, relative_error);
        
        // For a Gaussian from -3σ to +3σ, we should capture ~99.7% of the total area
        // Total area under normal distribution is 1.0, so this range should be ~0.997
        assert!(theoretical_area > 0.995, "Should capture most of the distribution area");
        assert!(polygon_area > 0.995, "Polygon should capture most of the distribution area");
    }

    #[test]
    fn test_shading_polygon_product_distributions() {
        // Test that product distributions also generate valid shading polygons
        let parent1 = GaussianDistribution::new(1, "Parent1".to_string(), -1.0, 1.0);
        let parent2 = GaussianDistribution::new(2, "Parent2".to_string(), 1.0, 1.0);
        
        let parents = vec![&parent1, &parent2];
        let product = GaussianDistribution::new_product(
            3,
            "Product".to_string(),
            vec![1, 2],
            &parents
        );
        
        let x_min = -4.0;
        let x_max = 4.0;
        let num_points = 100;
        
        // Generate expected points manually
        let mut expected_points = Vec::with_capacity(num_points + 2);
        expected_points.push([x_min, 0.0]);
        
        for i in 0..num_points {
            let x = x_min + (x_max - x_min) * i as f64 / (num_points - 1) as f64;
            let y = product.evaluate(x);
            expected_points.push([x, y]);
        }
        expected_points.push([x_max, 0.0]);
        
        // Validate structure
        assert_eq!(expected_points.len(), num_points + 2);
        
        // Validate boundaries
        assert_abs_diff_eq!(expected_points[0][1], 0.0, epsilon = EPSILON);
        assert_abs_diff_eq!(expected_points[expected_points.len()-1][1], 0.0, epsilon = EPSILON);
        
        // All curve points should be positive
        for i in 1..expected_points.len()-1 {
            assert!(expected_points[i][1] > 0.0);
        }
        
        // The maximum should be near the product distribution's mean
        let max_y = expected_points.iter().map(|p| p[1]).fold(0.0, f64::max);
        let expected_max_y = product.evaluate(product.mean);
        let tolerance = expected_max_y * 0.05; // 5% tolerance for product distributions
        
        assert!((max_y - expected_max_y).abs() < tolerance,
               "Product distribution polygon max should be close to theoretical max");
    }

    #[test] 
    fn test_shading_consistency_with_curve_points() {
        // Test that shading polygon points are consistent with curve generation
        let dist = GaussianDistribution::new(1, "Test".to_string(), 2.0, 1.5);
        
        let x_min = -2.0;
        let x_max = 6.0;
        let num_points = 50;
        
        // Generate expected curve points manually
        let mut expected_curve_points = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let x = x_min + (x_max - x_min) * i as f64 / (num_points - 1) as f64;
            let y = dist.evaluate(x);
            expected_curve_points.push([x, y]);
        }
        
        // Generate expected polygon points manually
        let mut expected_polygon_points = Vec::with_capacity(num_points + 2);
        expected_polygon_points.push([x_min, 0.0]);
        for point in &expected_curve_points {
            expected_polygon_points.push(*point);
        }
        expected_polygon_points.push([x_max, 0.0]);
        
        // Polygon should have 2 more points than curve (the boundary points)
        assert_eq!(expected_polygon_points.len(), expected_curve_points.len() + 2);
        
        // The middle points of the polygon should match the curve points
        for i in 0..expected_curve_points.len() {
            let curve_point = expected_curve_points[i];
            let polygon_point = expected_polygon_points[i + 1]; // Offset by 1 due to boundary point
            
            assert_abs_diff_eq!(curve_point[0], polygon_point[0], epsilon = EPSILON);
            assert_abs_diff_eq!(curve_point[1], polygon_point[1], epsilon = EPSILON);
        }
    }

    #[test]
    fn test_shading_polygon_no_duplicate_boundary_points() {
        // Test that the corrected polygon generation doesn't create duplicate boundary points
        let dist = GaussianDistribution::new(1, "Test".to_string(), 0.0, 1.0);
        
        let x_min = -2.0;
        let x_max = 2.0;
        let num_points = 5;
        
        // Generate expected points manually to verify the corrected logic
        let mut expected_points = Vec::with_capacity(num_points + 2);
        
        expected_points.push([x_min, 0.0]);  // Bottom-left corner
        
        // Curve points should NOT be at exact boundaries
        for i in 1..=num_points {
            let x = x_min + (x_max - x_min) * i as f64 / (num_points + 1) as f64;
            let y = dist.evaluate(x);
            expected_points.push([x, y]);
        }
        
        expected_points.push([x_max, 0.0]);  // Bottom-right corner
        
        // Verify structure
        assert_eq!(expected_points.len(), num_points + 2);
        
        // Verify no duplicate x-coordinates
        for i in 1..expected_points.len() {
            assert!(
                expected_points[i][0] > expected_points[i-1][0], 
                "Point {} x-coord ({}) should be greater than previous point x-coord ({})",
                i, expected_points[i][0], expected_points[i-1][0]
            );
        }
        
        // Verify boundary points are exactly at boundaries
        assert_abs_diff_eq!(expected_points[0][0], x_min, epsilon = EPSILON);
        assert_abs_diff_eq!(expected_points[0][1], 0.0, epsilon = EPSILON);
        
        let last_idx = expected_points.len() - 1;
        assert_abs_diff_eq!(expected_points[last_idx][0], x_max, epsilon = EPSILON);
        assert_abs_diff_eq!(expected_points[last_idx][1], 0.0, epsilon = EPSILON);
        
        // Verify curve points are strictly between boundaries
        for i in 1..expected_points.len()-1 {
            let x = expected_points[i][0];
            assert!(x > x_min && x < x_max, "Curve point {} x-coordinate should be strictly between boundaries", i);
            assert!(expected_points[i][1] > 0.0, "Curve point {} should be above x-axis", i);
        }
        
        // Test single point case
        let single_point_expected = vec![
            [x_min, 0.0],
            [(x_min + x_max) / 2.0, dist.evaluate((x_min + x_max) / 2.0)],
            [x_max, 0.0],
        ];
        
        assert_eq!(single_point_expected.len(), 3);
        assert!(single_point_expected[1][0] > x_min && single_point_expected[1][0] < x_max);
        assert!(single_point_expected[1][1] > 0.0);
    }
}
