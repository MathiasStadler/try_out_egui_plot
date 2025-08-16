// FROM HERE
// https://deepwiki.com/emilk/egui_plot/2-getting-started

use egui_plot::{Line, Plot, PlotPoints,PlotUi};

fn main(){
// Create points for a sine wave
let sin: PlotPoints = (0..1000).map(|i| {
    let x = i as f64 * 0.01;
    [x, x.sin()]
}).collect();

// Create a line item with those points
let line = Line::new("sin", sin);

// Create and show the plot, adding the line item via the PlotUi
Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));

}