# Histograms

Counters, gauges... just one more metric type to go: histograms!

Histograms are designed to capture (some) statistical properties of a set of observed values.
E.g. you might want to know the 99th percentile of your API latency in order to fire an alert
if it exceeds a certain threshold.

The precise representation of histograms (both in-memory and in the exported data) depends
on the recorder you will be using.
In Prometheus, for example, you have to specify a set of bins and the recorder will count
how many values fall into each bin.
Since the `metrics` crate is backend-agnostic, it doesn't expose APIs to configure
backend-specific behaviour. You need to check your recorder of choice to perform those
customizations (e.g. the `PrometheusRecorderBuilder` exposes a few methods to customise
the set of default bins as well as per-metric bins).

The observed value for a histogram is expected to be a float, rather than an integer.
`metrics` automatically converts `Duration`s into floats, since tracking how long something
takes is the bread and butter of histograms!

Everything we learned about counters and gauges applies to histograms as well
(description, units, labels).

