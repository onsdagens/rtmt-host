# Real-Time Monitoring and Trace - Host-side Tooling

This library provides host-side utilities for capturing and decoding Real-Time Monitoring and Trace(RTMT) frames (as implemented for the [Hippomenes reference architecture](https://github.com/perlindgren/hippomenes)).

## Background

Real-Time Monitoring and Trace is a proposed extension for the RISC-V architecture providing zero-overhead tracing of hardware events and preemptive framing of communications with the host.

In the original implementation of RTMT, changes in execution priority (be it interrupt dispatch/return or resource lock/unlock) are monitored, sending a cycle-accurate timestamp of the event over UART.

The preemptive framing relies on the [NCOBS protocol](https://github.com/perlindgren/rtmt), starting a new frame whenever the execution priority increases, and ending it whenever the execution priority decreases.

For more information on the core ideas behind RTMT, see the [original repo](https://github.com/perlindgren/rtmt), which contains the Why3 NCOBS formalization.

The example in this repo is tailored towards the [Hippomenes](https://github.com/perlindgren/hippomenes) architecture reference implementation of RTMT, and assumes the UART peripheral to be running using default settings.

A paper on the NCOBS protocol and RTMT has been submitted, and will be linked here once published.

## Usage

The `receiver` example sets up a listener on some specified serial port, and decodes incoming RTMT frames on the fly.
