# B-Shift

## Description

The purpose of this project is to explore an open library for implementing eShift into EVs.

As EVs become more prominent and more accessible, I believe a majority of people who are very much into cars, into the art of driving, the art of working through gears, need some sort of motivation to convert over to an EV.
Currently, EVs leave a lot to be desired for people who enjoy the interactive aspect of driving, and I hope to explore that missing piece.
Of course, this is not automotive industry ready, but it should provide basic building blocks in case anyone comes across it.

This was originally inspired by Hyundai's IONIQ 5 N, which features paddle shifters, emulating a gearbox and traditional engine power curves.
When I found out about this, I was hooked. I thought this was the coolest idea in the world.
I am not a math wizard, so getting comfortable with the math will take a bit of time.
But I am incredibly determined to do this.

## Goals

- [ ] Create a library that provides the correct calculations for all the necessary units of measurement to emulate a gearbox and engine.

- [ ] Integrate with [enginesynth](https://github.com/joelev/enginesynth) to emulate various engine noises, that can be output to a system audio device.

- [ ] Expose a user-facing service to interface with, allowing people to plug in game controllers, steering wheels, etc.

- [ ] Expose an interface to interact with a CAN bus, allowing for robotics and mechanics enthusiasts to integrate the system into a model electric vehicle project.

- [ ] Put this whole system on a board.

_NOTE_

> I have the intention to put this on a board for embedding into hardware projects, but I have zero clue what I want to target right now.
> If you read this and have a suggestion, please open an issue. Thank you!

## Pieces

### bshift-core

Holds all the base logic and math for converting units, calculating power, rpm, torque.

### bshift-service 

The interaction point for external input. Will allow for game controller input and CAN bus.

### enginesynth

TODO: need to pull in `enginesynth` and convert it to interface with this.

## Current System Design
