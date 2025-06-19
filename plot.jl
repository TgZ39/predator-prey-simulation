using JSON
using Plots
gr()
using Statistics
using Measures

function load_data(filename::String)
    open(filename, "r") do file
        return JSON.parse(file)
    end
end

function extract_series(data)
    num_predator = [entry[1] for entry in data]
    num_prey = [entry[2] for entry in data]
    num_empty = [entry[3] for entry in data]

    return num_predator, num_prey, num_empty
end

function plot_data(num_predator, num_prey, num_empty; output_file="population_plot.png")
    total = num_predator .+ num_prey .+ num_empty
    predator_frac = (num_predator ./ total) .* 100
    prey_frac = (num_prey ./ total) .* 100

    avg_predator = mean(predator_frac)
    avg_prey = mean(prey_frac)

    t = 1:length(num_predator)
    plot(
        t, [predator_frac, prey_frac],
        label = ["Räuber" "Beute"],
        lw = 3,
        fill = :auto,
        seriescolor = [:red :green],
        title = "Räuber-Beute-Beziehung Simulation",
        xlabel = "Zeit in Schritten",
        ylabel = "% der Felder",
        legend = :bottomright,
        size = (2400, 1200);
        titlefont = 28,
        guidefont = 24,
        tickfont = 20,
        legendfontsize = 18,
        gridlinewidth = 1,
        margin = 20mm,
    )

    hline!(
        [avg_predator],
        label = "Durchschnitt Räuber",
        color = :red,
        linestyle = :dot,
        lw = 3
    )
    hline!(
        [avg_prey],
        label = "Durchschnitt Beute",
        color = :green,
        linestyle = :dot,
        lw = 3
    )

    savefig(output_file)
end


filename = "data.json"
data = load_data(filename)

num_predator, num_prey, num_empty = extract_series(data)
plot_data(num_predator, num_prey, num_empty)
