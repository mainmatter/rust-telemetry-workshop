# Gauges

We have seen the end-to-end journey of a metric series. It's time to take a step back
and expand our toolset beyond counters.

We'll start with gauges.
Counters are expected to be positive and monotonically increasing: that's not the case for
gauges.
Gauges are designed to represent the current value of some property of interest—the number of
concurrent requests being handled by your API, the number of idle connections in your
database connection pool or the available disk space on your server.
The value can increase or decrease over time, therefore counters are ill-suited for these
kinds of measurements.

The value of a gauge is expected to be a float, rather than an integer.
Apart from that, everything else we learned about counters applies to gauges as well
(description, units, labels).
