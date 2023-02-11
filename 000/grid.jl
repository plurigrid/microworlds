using PowerSimulationsDynamics
using PowerSystems
using Sundials
using Plots

# Load data for the neighborhood
file_dir = "path/to/data"
neighborhood = System(joinpath(file_dir, "neighborhood.json"))

# Define time span for the simulation
tspan = (0.0, 24.0) # 24 hour simulation

# Define simulation for the neighborhood
sim = PSD.Simulation(
    ResidualModel, # Type of model used
    neighborhood, # System
    pwd(), # Folder to output results
    tspan, # Time span
)

# Get initial conditions for the simulation
x0_init = PSD.get_initial_conditions(sim)

# Run the simulation
PSD.execute!(
    sim, # Simulation structure
    IDA(), # Sundials DAE Solver
    dtmax = 0.1, # Maximum step size
)

# Read results from simulation
results = read_results(sim)

# Extract series for energy consumption from smart thermostats
thermostat_series = get_power_series(results, "smart_thermostat")

# Extract series for energy consumption from EV chargers
ev_charger_series = get_power_series(results, "ev_charger")

# Plot the results
plot(thermostat_series, label = "Smart Thermostats")
plot!(ev_charger_series, label = "EV Chargers")
xlabel!("Time [hr]")
ylabel!("Energy Consumption [kWh]")