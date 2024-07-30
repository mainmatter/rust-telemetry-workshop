# OpenTelemetry

While looking at JSON data in a terminal might be a good start, it's not very practical.  
Analyzing telemetry data using `jq` or scrolling through a terminal window is fairly tedious
and error-prone.\
In most real-world scenarios, we'll want to **export** our telemetry data to a centralized
system that provides advanced analysis capabilities.

In the old dark days, you would have had to pick a vendor and use their SDK to export your data
to their systems in whatever bespoke format they devised.\
Thankfully, the industry has moved on and we now have a vendor-neutral format for exporting
telemetry data: [OpenTelemetry](https://opentelemetry.io/). 
It has gained a significant foothold and it's now supported by most observability vendors as
a valid (if not preferred!) format for data ingestion.