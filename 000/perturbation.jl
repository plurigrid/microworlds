# LoadTrip Perturbation

# A LoadTrip perturbation due to a weather pattern can be modeled as follows:

# First, we can define the LoadTrip perturbation as a struct
mutable struct LoadTripWeather <: PowerSimulationsDynamics.LoadTrip
    time::Float64
    device::PowerSystems.ElectricLoad
    weather_pattern::String
end

# We can then create a function that takes in a simulation and weather pattern, and applies the LoadTrip perturbation
function apply_load_trip_weather(sim::PowerSimulationsDynamics.Simulation, weather_pattern::String)
    devices_to_trip = get_devices_affected_by_weather(sim, weather_pattern)
    for device in devices_to_trip
        PSD.apply_perturbation!(sim, LoadTripWeather(time=10.0, device=device, weather_pattern=weather_pattern))
    end
end

# The function `get_devices_affected_by_weather` would be specific to the simulation and system being modeled and would identify which devices are affected by the weather pattern. For example, in Northern California, a heat wave could cause a high demand for air conditioning, leading to a LoadTrip perturbation for air conditioning units.

# After defining the LoadTripWeather perturbation and the apply_load_trip_weather function, we can modify our simulation as follows:

# Run the simulation with the LoadTripWeather perturbation
PSD.execute!(
    sim, # Simulation structure
    IDA(), # Sundials DAE Solver
    dtmax = 0.1, # Maximum step size
)
apply_load_trip_weather(sim, "HeatWave")
PSD.execute!(
    sim, # Simulation structure
    IDA(), # Sundials DAE Solver
    dtmax = 0.1, # Maximum step size
)

# Read results from simulation after LoadTripWeather perturbation
results_with_load_trip = read_results(sim)

# Extract series for energy consumption from smart thermostats after LoadTripWeather perturbation
thermostat_series_with_load_trip = get_power_series(results_with_load_trip, "smart_thermostat")

# Extract series for energy consumption from EV chargers after LoadTripWeather perturbation
ev_charger_series_with_load_trip = get_power_series(results_with_load_trip, "ev_charger")

# Plot the results with LoadTripWeather perturbation
plot(thermostat_series, label = "Smart Thermostats (without load trip)")
plot!(ev_charger_series, label = "EV Chargers (without load trip)")
plot!(thermostat_series_with_load_trip, label = "Smart Thermostats (with load trip)")
plot!(ev_charger_series_with_load_trip, label = "EV Chargers (with load trip)")
xlabel!("Time [hr]")
