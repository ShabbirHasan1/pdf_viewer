# PDF Viewer - Complete Code Documentation

*A comprehensive line-by-line explanation of the Rust/egui PDF viewer application*

**Target Audience**: Developers with no prior Rust knowledge  
**Author Perspective**: Senior Rust developer with egui framework expertise

## Table of Contents
1. [Project Overview](#project-overview)
2. [Dependencies and Imports](#dependencies-and-imports)
3. [Application Entry Point](#application-entry-point)
4. [Core Data Structures](#core-data-structures)
5. [Mathematical Implementation](#mathematical-implementation)
6. [User Interface Logic](#user-interface-logic)
7. [Test Suite](#test-suite)

---

## Project Overview

This application is a **probability density function (PDF) explorer** built in Rust using the egui framework. It allows mathematicians to visualize and manipulate Gaussian distributions, multiply them together, and see real-time updates as parameters change.

**Key Concepts for Non-Rust Developers:**
- **`struct`**: Like a class in other languages, defines data structures
- **`impl`**: Implementation block, like methods in a class
- **`fn`**: Function declaration
- **`&`**: Reference (borrowing data without taking ownership)
- **`mut`**: Mutable (can be changed)
- **`Vec`**: Dynamic array/list
- **`HashMap`**: Dictionary/map data structure
- **`Option`**: Can contain a value or be empty (like nullable in other languages)
- **`Result`**: Success/Error return type

---

## Dependencies and Imports

```rust
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints, VLine};
use statrs::distribution::{Normal, Continuous};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
```

**Line 1**: `use eframe::egui;`
- **eframe**: The framework that handles window creation and event loops
- **egui**: The immediate mode GUI library for creating user interfaces
- This gives us access to UI widgets like buttons, sliders, text inputs

**Line 2**: `use egui_plot::{Line, Plot, PlotPoints, VLine};`
- **egui_plot**: Plotting library built on top of egui
- **Line**: Represents a curve/line in a plot
- **Plot**: The main plotting widget that displays graphs
- **PlotPoints**: Container for (x,y) coordinate data
- **VLine**: Vertical lines for marking specific x-values

**Line 3**: `use statrs::distribution::{Normal, Continuous};`
- **statrs**: Statistical functions library
- **Normal**: Implementation of the normal (Gaussian) distribution
- **Continuous**: Trait (interface) for continuous probability distributions
- This provides the mathematical functions for PDF calculations

**Line 4**: `use std::collections::HashMap;`
- **HashMap**: Built-in dictionary/map data structure
- Used to store distributions with unique IDs as keys
- Similar to `dict` in Python or `Map` in JavaScript

**Line 5**: `use serde::{Deserialize, Serialize};`
- **serde**: Serialization library for converting data to/from JSON
- **Serialize**: Trait for converting Rust data to JSON
- **Deserialize**: Trait for converting JSON to Rust data
- Enables saving/loading sessions

---

## Application Entry Point

```rust
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
```

**Line 7**: `fn main() -> Result<(), eframe::Error> {`
- Entry point of the program (like `main()` in C++ or `public static void main` in Java)
- Returns a `Result` type that can be either success `()` or an `eframe::Error`
- The `()` represents "unit type" (like `void` in C++)

**Lines 8-11**: Configuration object creation
- `let options = eframe::NativeOptions {`: Creates a configuration struct
- `viewport:`: Sets window properties
- `with_inner_size([1200.0, 800.0])`: Sets initial window size to 1200x800 pixels
- `..Default::default()`: Uses default values for all other fields (like `...` spread operator)

**Lines 13-17**: Application launch
- `eframe::run_native(`: Starts the application event loop
- `"PDF Viewer"`: Window title
- `options`: Configuration we created above
- `Box::new(|_cc| Ok(Box::new(PdfViewerApp::new())))`: 
  - Creates a closure (anonymous function) that initializes our app
  - `Box`: Heap allocation (like `new` in C++)
  - `_cc`: Creation context (unused, hence the `_` prefix)
  - `PdfViewerApp::new()`: Creates our main application instance

---

## Core Data Structures

### Main Application State

```rust
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
```

**Line 20**: `#[derive(Default)]`
- **Attribute/Annotation**: Tells Rust to automatically implement the `Default` trait
- `Default` provides a `default()` method that creates the struct with default values
- Similar to a parameterless constructor

**Line 21**: `struct PdfViewerApp {`
- Main application state container
- Holds all data needed for the entire application lifetime

**Line 22**: `distributions: HashMap<u32, GaussianDistribution>,`
- **Type**: Dictionary mapping unsigned 32-bit integers to distribution objects
- **Purpose**: Stores all probability distributions by unique ID
- **Key**: `u32` - unique identifier for each distribution
- **Value**: `GaussianDistribution` - the actual distribution data

**Line 23**: `next_id: u32,`
- **Type**: Unsigned 32-bit integer
- **Purpose**: Counter to ensure each new distribution gets a unique ID
- Incremented each time a new distribution is created

**Line 24**: `selected_for_multiplication: Vec<u32>,`
- **Type**: Dynamic array of unsigned 32-bit integers
- **Purpose**: Tracks which distributions the user has selected for multiplication
- Contains the IDs of selected distributions

**Line 25**: `plot_bounds: Option<egui_plot::PlotBounds>,`
- **Type**: Optional plot boundary object
- **Purpose**: Stores current zoom/pan state of the plot
- `Option` means it can be `Some(bounds)` or `None` (no custom bounds set)

**Line 26**: `show_shading: bool,`
- **Type**: Boolean (true/false)
- **Purpose**: Controls whether to draw filled areas under curves

**Line 27**: `shading_opacity: f32,`
- **Type**: 32-bit floating point number
- **Purpose**: Controls transparency of the shading (0.0 = invisible, 1.0 = opaque)

**Line 28**: `show_std_markers: bool,`
- **Type**: Boolean
- **Purpose**: Controls whether to draw vertical lines at standard deviation intervals

### Session Data Structure

```rust
#[derive(Serialize, Deserialize)]
struct SessionData {
    distributions: HashMap<u32, GaussianDistribution>,
    next_id: u32,
    show_shading: bool,
    shading_opacity: f32,
    show_std_markers: bool,
}
```

**Line 31**: `#[derive(Serialize, Deserialize)]`
- **Attributes**: Auto-generate JSON conversion code
- **Serialize**: Can convert this struct to JSON string
- **Deserialize**: Can create this struct from JSON string

**Lines 32-38**: Session data structure
- **Purpose**: Subset of app state that can be saved/loaded
- Contains all user data but excludes temporary UI state
- Notice it doesn't include `selected_for_multiplication` or `plot_bounds` 
- These are considered temporary UI state, not part of saved sessions

### App Initialization

```rust
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
```

**Line 40**: `impl PdfViewerApp {`
- **Implementation block**: Where we define methods for the `PdfViewerApp` struct
- Like defining methods inside a class

**Line 41**: `fn new() -> Self {`
- **Constructor function**: Creates a new instance of `PdfViewerApp`
- `Self` refers to `PdfViewerApp` (the type we're implementing for)
- By convention, Rust uses `new()` for constructors

**Lines 42-47**: Initialization with custom defaults
- Sets specific default values for visual settings
- `show_shading: true`: Enable shading by default
- `shading_opacity: 0.3`: 30% opacity for fills
- `show_std_markers: true`: Show standard deviation markers
- `..Default::default()`: Use default values for all other fields

### Gaussian Distribution Structure

```rust
#[derive(Clone, Serialize, Deserialize)]
struct GaussianDistribution {
    id: u32,
    name: String,
    mean: f64,
    std_dev: f64,
    parent_ids: Vec<u32>,
    is_product: bool,
}
```

**Line 51**: `#[derive(Clone, Serialize, Deserialize)]`
- **Clone**: Can create copies of this struct
- **Serialize/Deserialize**: Can convert to/from JSON for saving

**Line 52**: `struct GaussianDistribution {`
- Represents a single Gaussian (normal) distribution
- Contains both mathematical parameters and metadata

**Line 53**: `id: u32,`
- **Unique identifier**: Each distribution has a distinct ID
- Used as key in the HashMap for fast lookup

**Line 54**: `name: String,`
- **Display name**: Human-readable label (e.g., "Gaussian 1", "Product 3")
- `String` is owned, growable text (like `std::string` in C++)

**Line 55**: `mean: f64,`
- **Mathematical parameter**: Center of the bell curve
- `f64` is 64-bit floating point (double precision)

**Line 56**: `std_dev: f64,`
- **Mathematical parameter**: Width/spread of the bell curve
- Standard deviation - smaller values = narrower curves

**Line 57**: `parent_ids: Vec<u32>,`
- **Dependency tracking**: IDs of distributions used to create this one
- Empty for basic distributions, populated for product distributions

**Line 58**: `is_product: bool,`
- **Type flag**: `true` if this distribution is the product of others
- `false` for manually created distributions
- Affects UI (product distributions show different controls)

### Default Implementation for GaussianDistribution

```rust
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
```

**Line 61**: `impl Default for GaussianDistribution {`
- Implements the `Default` trait manually (custom default values)
- Required because we have specific default values different from Rust's built-in defaults

**Lines 63-70**: Default values
- `id: 0`: Default ID (will be overridden in practice)
- `name: "Gaussian 1".to_string()`: Creates owned string
- `mean: 0.0`: Standard normal distribution center
- `std_dev: 1.0`: Unit standard deviation
- `parent_ids: vec![]`: Empty vector (no parents)
- `is_product: false`: Not a product distribution

---

## Mathematical Implementation

### Basic Distribution Constructor

```rust
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
```

**Line 74**: `impl GaussianDistribution {`
- Implementation block for distribution methods

**Line 75**: `fn new(id: u32, name: String, mean: f64, std_dev: f64) -> Self {`
- **Constructor**: Creates a basic (non-product) distribution
- Takes ID, name, mathematical parameters
- Returns a new `GaussianDistribution` instance

**Lines 76-83**: Field initialization
- Uses Rust's field shorthand syntax (when variable name = field name)
- `id,` is equivalent to `id: id,`
- Sets `parent_ids` to empty and `is_product` to false

### Product Distribution Constructor

```rust
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
```

**Line 86**: Function signature
- Creates a distribution that's the product of multiple parent distributions
- `parents: &[&GaussianDistribution]` - slice of references to parent distributions
- `&[&T]` is a "slice of references" - like an array view in other languages

**Lines 87-88**: Mathematical explanation comments
- When you multiply Gaussian PDFs, the result is another Gaussian
- But with different mean and variance calculated from the parents

**Line 89**: `let (mean, variance) = Self::multiply_gaussians(parents);`
- **Tuple destructuring**: Gets two values from the function call
- `Self::multiply_gaussians` calls the static method on this type
- Returns the calculated mean and variance for the product

**Line 94**: `std_dev: variance.sqrt(),`
- **Square root**: Standard deviation is the square root of variance
- Mathematical relationship: σ = √(σ²)

### Gaussian Multiplication Mathematics

```rust
fn multiply_gaussians(gaussians: &[&GaussianDistribution]) -> (f64, f64) {
    if gaussians.is_empty() {
        return (0.0, 1.0);
    }
    
    // For multiplying Gaussian PDFs: 
    // The product of two Gaussians N(μ₁,σ₁²) * N(μ₂,σ₂²) is proportional to
    // N((μ₁/σ₁² + μ₂/σ₂²)/(1/σ₁² + 1/σ₂²), 1/(1/σ₁² + 1/σ₂²))
    
    let mut precision_sum = 0.0;  // sum of 1/σ²
    let mut weighted_mean_sum = 0.0;  // sum of μ/σ²
```

**Line 100**: Function signature
- **Static method**: Doesn't need an instance, called on the type itself
- Takes slice of references to distributions
- Returns tuple of (mean, variance)

**Lines 101-103**: Edge case handling
- **Early return**: If no gaussians provided, return default values
- Prevents division by zero errors later

**Lines 105-107**: Mathematical explanation
- **Mathematical comment**: Explains the theory behind Gaussian multiplication
- Uses Unicode mathematical symbols for clarity (μ = mean, σ = standard deviation)

**Lines 109-110**: Variable initialization
- **precision**: Mathematical term = 1/variance = 1/σ²
- **weighted_mean**: mean weighted by precision = μ/σ²
- These are the building blocks for the final calculation

```rust
    for gaussian in gaussians {
        let precision = 1.0 / (gaussian.std_dev * gaussian.std_dev);
        precision_sum += precision;
        weighted_mean_sum += gaussian.mean * precision;
    }
    
    let result_mean = weighted_mean_sum / precision_sum;
    let result_variance = 1.0 / precision_sum;
    
    (result_mean, result_variance)
```

**Lines 112-116**: Accumulation loop
- **For each parent distribution**: Calculate its contribution
- `precision = 1.0 / (σ²)`: Higher precision = narrower distribution = more influence
- Accumulate both precision values and weighted means

**Lines 118-119**: Final calculation
- **Result mean**: Weighted average of parent means (weights = precisions)
- **Result variance**: Inverse of total precision
- This is the mathematical formula for multiplying Gaussian PDFs

**Line 121**: Return tuple
- Returns both calculated values as a tuple
- Will be destructured by the calling code

### PDF Evaluation

```rust
fn evaluate(&self, x: f64) -> f64 {
    let normal = Normal::new(self.mean, self.std_dev).unwrap();
    normal.pdf(x)
}
```

**Line 124**: `fn evaluate(&self, x: f64) -> f64 {`
- **Instance method**: Takes `&self` (reference to the current distribution)
- **Purpose**: Calculate the probability density at point x
- Core function for plotting and calculations

**Line 125**: `let normal = Normal::new(self.mean, self.std_dev).unwrap();`
- **Creates normal distribution**: Using the `statrs` library
- `Normal::new()` returns `Result<Normal, Error>`
- `.unwrap()` extracts the value (panics if error - acceptable here since parameters should be valid)

**Line 126**: `normal.pdf(x)`
- **PDF calculation**: Probability Density Function at point x
- Uses the mathematical formula: f(x) = (1/(σ√(2π))) * e^(-½((x-μ)/σ)²)
- Returns the height of the bell curve at position x

### Point Generation for Plotting

```rust
fn generate_points(&self, x_min: f64, x_max: f64, num_points: usize) -> PlotPoints {
    let mut points = Vec::new();
    for i in 0..num_points {
        let x = x_min + (x_max - x_min) * i as f64 / (num_points - 1) as f64;
        let y = self.evaluate(x);
        points.push([x, y]);
    }
    PlotPoints::new(points)
}
```

**Line 129**: Function signature
- **Purpose**: Generate points for drawing smooth curves
- Takes x-range and desired number of points
- Returns `PlotPoints` (egui's plotting data structure)

**Line 130**: `let mut points = Vec::new();`
- **Mutable vector**: Can grow and change
- Will hold array of [x, y] coordinate pairs

**Lines 131-135**: Point generation loop
- **Linear interpolation**: Evenly space points across x-range
- `i as f64`: Type conversion from integer to float
- **Formula**: `x = x_min + (range * progress)`
- `progress = i / (num_points - 1)` ranges from 0.0 to 1.0

**Line 133**: `let y = self.evaluate(x);`
- **Calculate PDF value**: Get bell curve height at each x position
- Uses the `evaluate` method we defined above

**Line 134**: `points.push([x, y]);`
- **Add coordinate pair**: Array literal `[x, y]`
- Building up the list of points for plotting

**Line 136**: `PlotPoints::new(points)`
- **Convert to plot format**: Wrap our points in egui's plotting structure
- This object can be passed to egui's plotting functions

### Shading Polygon Generation

```rust
fn generate_shading_polygon(&self, x_min: f64, x_max: f64, num_points: usize) -> PlotPoints {
    let mut points = Vec::with_capacity(num_points + 2);
    
    // Create clean polygon: bottom-left -> curve points -> bottom-right
    // Key insight: don't duplicate corner points in the curve sampling
    
    points.push([x_min, 0.0]);  // Bottom-left corner
```

**Line 139**: Function signature
- **Purpose**: Generate points for filled area under curve
- Similar to `generate_points` but creates a closed polygon

**Line 140**: `let mut points = Vec::with_capacity(num_points + 2);`
- **Pre-allocated vector**: Reserves memory for efficiency
- `+ 2` accounts for the two boundary points at y=0

**Lines 142-143**: Algorithm explanation
- **Strategy**: Create polygon that can be filled
- Must form closed shape: bottom-left → curve → bottom-right → (automatically closes)

**Line 145**: `points.push([x_min, 0.0]);`
- **Bottom-left corner**: Start polygon at x-axis
- This creates the "floor" of the filled area

```rust
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
```

**Lines 147-153**: Single point case
- **Edge case**: When only one curve point requested
- **Strategy**: Place single point at center of range
- Ensures meaningful representation even with minimal points

**Lines 154-160**: Multiple points case
- **Improved spacing**: Points are spaced *between* boundaries, not *at* boundaries
- **Formula change**: Divides by `(num_points + 1)` instead of `(num_points - 1)`
- **Range**: `i` goes from 1 to `num_points` inclusive
- **Effect**: Avoids duplicate x-coordinates with boundary points

**Line 162**: `points.push([x_max, 0.0]);`
- **Bottom-right corner**: Complete the polygon
- Now we have: bottom-left → curve points → bottom-right
- Rendering system will automatically close back to first point

### Debug Point Generation

```rust
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
```

**Lines 168-188**: Debug version of polygon generation
- **Purpose**: Testing and debugging - returns raw Vec instead of PlotPoints
- **Identical logic**: Same algorithm as `generate_shading_polygon`
- **Key difference**: Returns `Vec<[f64; 2]>` instead of `PlotPoints`
- **Why needed**: PlotPoints is opaque - we can't inspect its contents in tests
- **Return type**: `Vec<[f64; 2]>` - vector of 2-element arrays (x, y coordinates)

### Standard Deviation Markers

```rust
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
```

**Line 191**: `fn get_std_markers(&self) -> Vec<f64> {`
- **Purpose**: Calculate x-positions for vertical marker lines
- **Statistical significance**: -3σ, -2σ, -1σ, μ, +1σ, +2σ, +3σ

**Lines 192-200**: Marker calculations
- **Mathematical meaning**: 
  - ±1σ: ~68% of data falls within this range
  - ±2σ: ~95% of data falls within this range  
  - ±3σ: ~99.7% of data falls within this range
- **Visual purpose**: Help users understand the distribution's spread
- **Return**: Vector of x-coordinates where vertical lines should be drawn

---

## Application Logic Implementation

### Product Distribution Updates

```rust
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
```

**Line 204**: `impl PdfViewerApp {`
- Implementation block for main application methods

**Line 205**: `fn update_product_distributions(&mut self) {`
- **Purpose**: Recalculate product distributions when parent parameters change
- **Mutable reference**: `&mut self` allows modifying the app state

**Line 206**: `let mut updates = Vec::new();`
- **Update queue**: Collect changes before applying them
- **Why separate?**: Can't modify HashMap while iterating over it (borrowing rules)

**Lines 208-219**: Find product distributions needing updates
- **Line 208**: Iterate over all distributions
- **Line 209**: Check if it's a product with parents
- **Lines 210-213**: Collect references to parent distributions
  - `filter_map`: Try to get each parent, keep only successful lookups
  - Handles case where parent might have been deleted
- **Line 215**: Verify all parents still exist
- **Line 216**: Calculate new parameters
- **Line 217**: Queue the update (ID, new_mean, new_std_dev)

**Lines 221-226**: Apply updates
- **Separate loop**: Now safe to modify distributions
- **Line 222**: `if let Some(dist) = ...` - safe pattern matching
- Only updates if distribution still exists
- **Lines 223-224**: Apply the calculated changes

### Plot Range Calculation

```rust
fn get_plot_range(&self) -> (f64, f64) {
    if let Some(bounds) = &self.plot_bounds {
        (bounds.min()[0], bounds.max()[0])
    } else {
        (-6.0, 6.0)
    }
}
```

**Line 230**: `fn get_plot_range(&self) -> (f64, f64) {`
- **Purpose**: Determine current x-axis range for plotting
- **Returns**: Tuple of (x_min, x_max)

**Line 231**: `if let Some(bounds) = &self.plot_bounds {`
- **Pattern matching**: Check if custom bounds are set
- **Borrowing**: `&self.plot_bounds` borrows the Option
- **Destructuring**: If Some, extract the bounds value

**Line 232**: `(bounds.min()[0], bounds.max()[0])`
- **Extract x-range**: bounds contains both x and y ranges
- `[0]` gets the x-coordinate from the min/max points
- Returns the custom zoom/pan range

**Line 234**: `(-6.0, 6.0)`
- **Default range**: ±6 standard deviations
- **Mathematical rationale**: Captures >99.9% of normal distribution
- Used when no custom zoom/pan has been applied

### Auto-fit View Calculation

```rust
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
```

**Line 238**: `fn auto_fit_view(&mut self) {`
- **Purpose**: Automatically calculate optimal view bounds
- **Smart fitting**: Ensures all distributions are visible

**Lines 239-241**: Early return for empty case
- **Guard clause**: No distributions = nothing to fit
- Prevents errors in the calculations below

**Lines 243-245**: Initialize tracking variables
- **min_mean/max_mean**: Track leftmost and rightmost distribution centers
- **max_std_dev**: Track widest distribution
- **Infinity constants**: Ensure first comparison always updates the value

**Lines 247-251**: Find bounds across all distributions
- **Iterate**: Check every distribution
- **min()**: Keep the smaller value (leftward)
- **max()**: Keep the larger value (rightward/wider)

**Lines 253-255**: Calculate x-axis range
- **Margin**: 4 standard deviations beyond the extreme means
- **Mathematical rationale**: Shows the tails of even wide distributions
- **Coverage**: Ensures >99.99% of each distribution is visible

**Line 258**: Calculate y-axis maximum
- **PDF maximum**: The peak height of the narrowest distribution
- **Formula**: 1/(σ√(2π)) - maximum value of normal distribution
- **1.1 multiplier**: 10% extra space above the peak

**Lines 260-263**: Set the calculated bounds
- **PlotBounds::from_min_max**: egui plotting function
- **Bottom-left**: `[x_min, 0.0]` - x range starts at calculated min, y at zero
- **Top-right**: `[x_max, y_max]` - x range ends at calculated max, y at calculated max

### Session Management

```rust
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
```

**Line 267**: `fn save_session(&self) -> Result<String, String> {`
- **Purpose**: Convert current app state to JSON string
- **Return type**: `Result<String, String>` - either JSON or error message

**Lines 268-274**: Create session data object
- **Selective copying**: Only save persistent state
- **Excludes**: UI state like `selected_for_multiplication`, `plot_bounds`
- **Clone distributions**: Deep copy of the HashMap

**Lines 276-277**: JSON serialization
- **serde_json::to_string_pretty**: Converts to formatted JSON
- **map_err**: Converts serialization error to our error format
- **Error handling**: Transforms technical error into user-friendly message

```rust
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
```

**Line 280**: `fn load_session(&mut self, json_data: &str) -> Result<(), String> {`
- **Purpose**: Restore app state from JSON string
- **Mutable**: Modifies the current app state
- **Parameters**: `&str` - string slice (borrowed string)

**Lines 281-282**: JSON deserialization
- **serde_json::from_str**: Parse JSON into SessionData
- **?**: Error propagation - if parsing fails, return the error immediately
- **Type annotation**: `: SessionData` tells Rust what type to deserialize into

**Lines 284-288**: Restore app state
- **Direct assignment**: Replace current state with loaded values
- **Field by field**: Ensures all persistent state is restored

**Line 289**: `self.selected_for_multiplication.clear();`
- **Clear selection**: UI state shouldn't persist across sessions
- **Fresh start**: User begins with no selections

**Line 291**: `Ok(())`
- **Success return**: Indicates successful loading
- **Unit value**: `()` represents "no meaningful return value"

---

## User Interface Logic

### Main Update Loop

```rust
impl eframe::App for PdfViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
```

**Line 295**: `impl eframe::App for PdfViewerApp {`
- **Trait implementation**: Makes our app compatible with eframe
- **eframe::App**: Interface that eframe expects for applications

**Line 296**: `fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {`
- **Core UI method**: Called every frame to build the interface
- **ctx**: egui context - provides access to all UI functions
- **_frame**: Frame information (unused, hence `_` prefix)

**Line 297**: `egui::CentralPanel::default().show(ctx, |ui| {`
- **Central panel**: Main area of the window (excludes menu bars, side panels)
- **show()**: Displays the panel and runs the closure to build its contents
- **Closure**: `|ui|` - anonymous function that receives a UI builder object

### Header Section

```rust
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
```

**Line 298**: `ui.horizontal(|ui| {`
- **Horizontal layout**: Child widgets arranged left-to-right
- **UI scope**: The closure receives a new UI context for this layout

**Line 299**: `ui.label("PDF Viewer - Probability Density Function Explorer");`
- **Text label**: Static text display
- **Purpose**: Application title/header

**Line 301**: `ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {`
- **Custom layout**: Override default left-to-right layout
- **Right-to-left**: Buttons appear on the right side
- **Center align**: Vertically center the buttons

**Lines 302-312**: Save session button
- **Button with emoji**: `💾` provides visual cue
- **clicked()**: Returns true if button was pressed this frame
- **Match expression**: Pattern match on save result
- **Success case**: Copy JSON to system clipboard
- **Error case**: Print error to console (stderr)

**Lines 314-317**: Load session button
- **Placeholder implementation**: Currently just prints instruction
- **Note**: Could be enhanced with file dialog or text input

### Initial Distribution Creation

```rust
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
```

**Lines 323-333**: Automatic first distribution
- **User experience**: Ensure app is never empty
- **Standard normal**: Mean=0, std_dev=1 (mathematical default)
- **Naming**: "Gaussian 1", "Gaussian 2", etc.
- **ID management**: Use and increment next_id counter

### Main Layout - Controls and Plot

```rust
            ui.horizontal(|ui| {
                // Left panel for controls
                ui.vertical(|ui| {
                    ui.set_width(300.0);
                    ui.heading("Distribution Controls");
```

**Line 335**: `ui.horizontal(|ui| {`
- **Two-column layout**: Controls on left, plot on right
- **Responsive**: egui automatically handles sizing

**Line 337**: `ui.vertical(|ui| {`
- **Left column**: Vertical stack of controls
- **Nested layouts**: horizontal contains vertical

**Line 338**: `ui.set_width(300.0);`
- **Fixed width**: Ensures controls don't get too wide/narrow
- **Pixels**: egui uses logical pixels (DPI-aware)

### Add Distribution Button

```rust
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
```

**Lines 341-350**: Add new distribution
- **Interactive button**: Immediately creates new distribution when clicked
- **Consistent naming**: Sequential numbering (Gaussian 1, 2, 3...)
- **Default parameters**: Standard normal distribution (mean=0, std_dev=1)
- **State update**: Add to HashMap and increment ID counter

### Visual Options Section

```rust
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
```

**Line 355**: `ui.heading("Visual Options");`
- **Section header**: Larger text to organize controls
- **UI organization**: Groups related controls together

**Line 356**: `ui.checkbox(&mut self.show_shading, "Show shading under curves");`
- **Mutable reference**: `&mut` allows checkbox to modify the value
- **Two-way binding**: Checkbox reflects current state and can change it

**Lines 357-362**: Conditional opacity slider
- **Dependent control**: Only show when shading is enabled
- **Horizontal layout**: Label and slider on same line
- **Slider range**: 0.0 to 1.0 (0% to 100% opacity)
- **fixed_decimals(2)**: Display as "0.75" instead of "0.7500000"

**Line 364**: Standard deviation markers checkbox
- **Similar pattern**: Toggle for showing statistical markers
- **Visual aid**: Helps understand distribution spread

### Multiplication Controls

```rust
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
```

**Line 369**: `ui.heading("Multiply PDFs");`
- **Mathematical feature**: Core functionality for exploring PDF products
- **Section organization**: Clear separation from visual controls

**Lines 371-391**: Multiply Selected button logic
- **Validation**: Requires at least 2 selected distributions
- **Safety check**: Verify selected distributions still exist
- **filter_map**: Skip any IDs that don't correspond to existing distributions
- **Product creation**: Use the mathematical constructor
- **Cleanup**: Clear selection after successful multiplication

### Clear Selection and Status

```ruby
                        if ui.button("Clear Selection").clicked() {
                            self.selected_for_multiplication.clear();
                        }
                    });
                    
                    if !self.selected_for_multiplication.is_empty() {
                        ui.label(format!("Selected: {} distributions", self.selected_for_multiplication.len()));
                    }
```

**Lines 393-396**: Clear Selection button
- **User control**: Allow users to reset their selection
- **Simple operation**: Just empty the selection vector

**Lines 398-400**: Selection status display
- **User feedback**: Show how many distributions are selected
- **Conditional**: Only show when there are selections
- **Dynamic text**: Updates in real-time as selection changes

### Distribution Parameter Controls

```rust
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
```

**Line 405**: `let mut to_remove = Vec::new();`
- **Deferred deletion**: Collect items to delete without modifying during iteration
- **Borrowing rules**: Can't modify HashMap while iterating over it

**Line 406**: `for (id, dist) in self.distributions.iter_mut() {`
- **Mutable iteration**: Allows modifying distribution parameters
- **Destructuring**: Gets both key (id) and value (dist) from HashMap

**Line 407**: `ui.group(|ui| {`
- **Visual grouping**: Draws border around each distribution's controls
- **Organization**: Makes it clear which controls belong to which distribution

**Lines 409-420**: Selection checkbox logic
- **Local variable**: `selected` tracks current state
- **Synchronization**: Update selection vector when checkbox changes
- **Add to selection**: Push ID if newly selected
- **Remove from selection**: Filter out ID if deselected
- **Duplicate prevention**: Check before adding to avoid duplicates

### Distribution Header and Delete

```rust
                                ui.label(&dist.name);
                                if dist.is_product {
                                    ui.label("(Product)");
                                }
                                if ui.small_button("✖").clicked() {
                                    to_remove.push(*id);
                                }
                            });
```

**Line 422**: `ui.label(&dist.name);`
- **Distribution name**: Display user-friendly name
- **Reference**: `&` borrows the string without taking ownership

**Lines 423-425**: Product indicator
- **Visual distinction**: Show when distribution is calculated from others
- **User understanding**: Helps identify which distributions are computed vs. manual

**Lines 426-428**: Delete button
- **Small button**: Less prominent to avoid accidental deletion
- **Cross symbol**: Universal delete/close icon
- **Deferred deletion**: Add to removal list instead of immediate deletion

### Parameter Controls for Manual Distributions

```rust
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
```

**Line 431**: `if !dist.is_product {`
- **Conditional controls**: Only manual distributions can be edited
- **Logic**: Product distributions are calculated from parents

**Lines 432-437**: Mean parameter control
- **DragValue**: Click and drag to change value
- **Speed**: How fast value changes when dragging (0.1 units per pixel)
- **Range**: Reasonable bounds for mathematical exploration

**Lines 439-444**: Standard deviation control
- **Slower speed**: 0.01 for finer control of distribution width
- **Positive range**: Standard deviation must be > 0
- **Upper bound**: 5.0 prevents extremely wide distributions

**Lines 446-450**: Slider alternatives
- **Dual interface**: Both drag values and sliders for user preference
- **Visual feedback**: Sliders show position within range
- **Accessibility**: Some users prefer sliders to drag values

### Product Distribution Display

```rust
                            } else {
                                // Show read-only info for product distributions
                                ui.label(format!("Mean: {:.3}", dist.mean));
                                ui.label(format!("Std Dev: {:.3}", dist.std_dev));
                                ui.label(format!("Parents: {:?}", dist.parent_ids));
                            }
```

**Lines 452-457**: Read-only product information
- **No editing**: Product distributions are calculated, not manual
- **Display values**: Show current calculated parameters
- **Precision**: 3 decimal places for readability
- **Parent tracking**: Show which distributions were used to create this one
- **Debug format**: `{:?}` displays the vector in a readable format

### Deletion and Updates

```rust
                    // Remove marked distributions
                    for id in to_remove {
                        self.distributions.remove(&id);
                        // Also remove from selection
                        self.selected_for_multiplication.retain(|&x| x != id);
                    }
                    
                    // Update product distributions when their parents change
                    self.update_product_distributions();
```

**Lines 461-466**: Execute deletions
- **Deferred deletion**: Now safe to remove from HashMap
- **Cleanup selection**: Remove deleted IDs from selection list
- **Consistency**: Maintain app state integrity

**Line 469**: `self.update_product_distributions();`
- **Real-time updates**: Recalculate products when parent parameters change
- **Mathematical consistency**: Ensures product distributions reflect current parent values
- **User experience**: Changes are immediately visible in the plot

### Plot Section

```rust
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
```

**Line 475**: `ui.vertical(|ui| {`
- **Right column**: Plot and its controls
- **Vertical layout**: Title, controls, then plot

**Line 476**: `ui.heading("Probability Density Functions");`
- **Plot title**: Mathematical term for what we're visualizing

**Lines 479-487**: Plot control buttons
- **Reset View**: Return to default zoom/pan (-6 to +6 range)
- **Auto-fit**: Automatically calculate optimal view
- **Usage instructions**: Tell users about mouse interactions

### Plot Configuration and Colors

```rust
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
```

**Lines 489-495**: Plot configuration
- **Unique ID**: "pdf_plot" identifies this plot widget
- **Aspect ratio**: 2.0 means width is twice the height
- **Interactive features**: Enable zoom, pan, and scroll
- **Axes**: Show both x and y axes with labels

**Lines 497-504**: Color palette
- **Cycle colors**: Each distribution gets a different color
- **Predefined colors**: Mix of standard and custom RGB colors
- **Visual distinction**: Helps users identify different distributions

### Shading Implementation

```rust
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
```

**Line 507**: `for (idx, dist) in self.distributions.values().enumerate() {`
- **Indexed iteration**: Get both the distribution and its index
- **Color cycling**: Index determines which color to use

**Line 508**: `let (x_min, x_max) = self.get_plot_range();`
- **Current view range**: Respects user's zoom/pan state
- **Dynamic range**: Only generate points for visible area

**Lines 512-531**: Shading implementation
- **Conditional**: Only draw shading when enabled by user
- **Point generation**: 300 points for smooth curves
- **Opacity calculation**: Convert from 0.0-1.0 to 0-255
- **Minimum alpha**: Prevent completely transparent (which triggers auto-coloring)
- **Fill method**: Use Line's native filling capability
- **Invisible stroke**: Only want the filled area, not the outline
- **Fill to baseline**: Area between curve and y=0

### Curve Drawing

```rust
                            // Draw the curve line
                            let points = dist.generate_points(x_min, x_max, 300);
                            let line = Line::new(points)
                                .name(&dist.name)
                                .color(color);
                            plot_ui.line(line);
```

**Lines 534-539**: Main curve drawing
- **Same points**: 300 points for smooth curves
- **Line object**: egui plotting primitive
- **Name**: Appears in plot legend
- **Color**: From the predefined palette
- **Draw**: Add line to the plot

### Standard Deviation Markers

```rust
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
```

**Lines 542-558**: Statistical markers
- **Conditional**: Only when user enables markers
- **7 markers**: -3σ, -2σ, -1σ, μ, +1σ, +2σ, +3σ
- **Visibility check**: Only draw markers within current view range
- **Style variation**: 
  - **Mean marker** (index 3): Thicker line (2.0 width)
  - **Std dev markers**: Thinner, dimmer lines (0.7 gamma = 70% brightness)
- **Dashed lines**: Visual distinction from solid curves
- **Color matching**: Each distribution's markers use its color

---

## Test Suite

The test suite contains 1337 lines of comprehensive testing code. Here's an overview of the key testing strategies:

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    const EPSILON: f64 = 1e-10;
    const APPROX_EPSILON: f64 = 1e-6;
```

**Lines 567-574**: Test module setup
- **Conditional compilation**: `#[cfg(test)]` only includes in test builds
- **Import parent scope**: `use super::*` brings in all our application code
- **Floating point comparison**: `approx` crate for comparing floating point numbers
- **Constants**: Different epsilon values for strict vs. approximate comparisons

### Mathematical Verification Tests

The test suite includes comprehensive verification of mathematical properties:

1. **Gaussian Distribution Creation** (lines 576-585)
   - Verifies constructor sets all fields correctly
   - Tests default values and type properties

2. **PDF Evaluation Tests** (lines 587-616)
   - Validates mathematical correctness against known formulas
   - Tests standard normal distribution (μ=0, σ=1)
   - Verifies symmetry properties
   - Tests custom parameters (μ=2, σ=0.5)

3. **Gaussian Multiplication Mathematics** (lines 618-661)
   - Tests precision-weighted mean calculation
   - Verifies variance combination formulas
   - Tests edge cases (empty input, single distribution)
   - Validates multi-distribution products

4. **Point Generation Tests** (lines 691-768)
   - Verifies curve smoothness and continuity
   - Tests shading polygon generation
   - Validates boundary conditions
   - Ensures proper point ordering

### Application State Tests

1. **Session Management** (lines 803-873)
   - Round-trip save/load testing
   - JSON serialization verification
   - Product distribution persistence
   - Error handling for invalid JSON

2. **Product Distribution Updates** (lines 875-914)
   - Real-time recalculation when parents change
   - Dependency tracking validation
   - Mathematical consistency checks

3. **Auto-fit View Calculation** (lines 975-1000)
   - Boundary calculation verification
   - Empty distribution handling
   - Multi-distribution range optimization

### Edge Case and Robustness Tests

1. **Extreme Parameter Testing** (lines 925-962)
   - Very small standard deviations (narrow distributions)
   - Very large standard deviations (wide distributions)
   - Numerical integration verification
   - Symmetry preservation

2. **Shading Polygon Validation** (lines 1040-1336)
   - Different distribution parameters
   - Edge cases (minimal points, large ranges)
   - Area approximation accuracy
   - Product distribution compatibility
   - Boundary point duplication prevention

### Key Testing Insights

**Mathematical Accuracy**: Tests verify that the mathematical formulas are implemented correctly, including:
- Normal distribution PDF calculations
- Gaussian multiplication formulas (precision-weighted means)
- Statistical marker positions (±1σ, ±2σ, ±3σ)

**Numerical Stability**: Tests ensure the application handles edge cases:
- Very narrow distributions (σ=0.01)
- Very wide distributions (σ=10.0)
- Empty datasets
- Single-point polygons

**User Interface Consistency**: Tests verify that:
- Session save/load preserves all important state
- Product distributions update when parents change
- Visual elements (shading, markers) generate correctly

**Performance Considerations**: The polygon generation tests specifically address:
- Efficient point generation algorithms
- Proper memory allocation (`Vec::with_capacity`)
- Boundary condition handling to prevent visual artifacts

---

## Summary

This PDF viewer application demonstrates several advanced Rust and mathematical concepts:

1. **Mathematical Modeling**: Implements Gaussian distribution mathematics with precision-weighted multiplication
2. **Real-time Visualization**: Uses egui for immediate-mode GUI with plotting capabilities
3. **State Management**: Complex application state with dependency tracking between distributions
4. **Session Persistence**: JSON serialization for save/load functionality
5. **Interactive UI**: Real-time parameter adjustment with immediate visual feedback
6. **Comprehensive Testing**: Extensive test suite covering mathematical accuracy and edge cases

The code follows Rust best practices including:
- **Memory Safety**: No manual memory management, borrowing system prevents data races
- **Error Handling**: Proper `Result` types for fallible operations
- **Type Safety**: Strong typing prevents many classes of bugs
- **Performance**: Efficient data structures and algorithms for real-time updates

This application serves as an excellent example of scientific computing in Rust, combining mathematical rigor with user-friendly interactive visualization.