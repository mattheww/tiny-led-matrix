#![no_std]

//! A library for direct control of a small monochrome LED display.
//!
//! This is designed to support the [micro:bit](https://microbit.org/)'s 5×5
//! display, but nothing in this crate is specific to the micro:bit.
//!
//! The library assumes the display is internally organised as a two-dimensional
//! matrix of rows and columns, with the individual LEDs addressed directly
//! (typically with one GPIO pin for each row and one for each column).
//!
//! It isn't designed for the kind of display where you “clock in” the data
//! for an LED row using a smaller number of output pins.
//!
//!
//! # Display model
//!
//! The LED display must be organised as a two-dimensional array of *matrix
//! rows* and *matrix columns*. These describe how the LEDs are wired up, and
//! need not match their visible arrangement.
//!
//! At any time, LEDs from at most one matrix row of the display are lit; the
//! display driver repeatedly lights LEDs from each row in turn to create the
//! illusion of a stable image.
//!
//! At present this crate supports at most 16 matrix columns. There's no
//! strict limit on the number of rows, but in practice it must be small so
//! that the LEDs are lit for a reasonable proportion of the time.
//!
//!
//! ## Greyscale model
//!
//! LED brightness levels are described using a scale from 0 (off) to 9
//! (brightest) inclusive.
//!
//! These are converted to time slices using the same relative durations as
//! the [micro:bit MicroPython port][micropython] uses.
//!
//! The time slice for each level above 1 is approximately 1.9× the slice for
//! the previous level.
//!
//! If there are three matrix rows in the display, an LED with brightness 9 is
//! lit for one third of the time.
//!
//!
//! # Configuring the library for a device.
//!
//! To use this library, you will have to supply implementations of a number
//! of traits, describing your device and its display.
//!
//!
//! ## Matrix
//!
//! You must supply an implementation of the [`Matrix`] trait to describe the
//! matrix dimensions, and how the matrix corresponds to the visible arrangement
//! of LEDs.
//!
//!
//! ## Images and Render
//!
//! The [`Render`] trait defines the interface that an image-like type needs to
//! provide in order to be displayed.
//!
//! The image reports the brightness to use for a given LED, given coordinates
//! according to the visible arrangement.
//!
//! This crate doesn't supply any implementations of `Render`; you should
//! define at least one image type of a suitable size and implement `Render`
//! for it.
//!
//!
//! ## Frames
//!
//! Types implementing [`Render`] are used to update a [`Frame`] (which is in
//! turn passed to a [`Display`]).
//!
//! A `Frame` instance is a 'compiled' representation of a greyscale image of
//! the size required by the display, in a form that's more directly usable by
//! the display code.
//!
//! This is exposed in the public API so that you can construct the `Frame`
//! representation in code running at a low priority. Then only
//! [`Display::set_frame()`] has to be called in code that can't be interrupted
//! by the display timer.
//!
//! You must supply an implementation of the [`Frame`] trait.
//!
//!
//! ## Timer control
//!
//! The `Display` expects to control a single timer which will generate
//! interrupts at appropriate times. [`Display::handle_event()`] is intended to
//! be called from these interrupts.
//!
//! You must supply an implementation of the [`DisplayTimer`] trait providing
//! the interface that the `Display` needs to control the timer.
//!
//! The [`DisplayTimer`] implementation determines the refresh rate for the
//! display.
//!
//! The `Display` requests an interrupt for the point in time when the next
//! row is due to be lit. When rendering greyscale images, it requests
//! additional interrupts within each row's time period. It only requests
//! interrupts for the greyscale levels which are required for what's
//! currently being displayed.
//!
//!
//! ## LED control
//!
//! The `Display` expects to be able to light an arbitrary subset of the LEDs in
//! a given matrix row.
//!
//! You must supply an implementation of the [`DisplayControl`] trait to provide
//! the interface that it needs.
//!
//!
//! # Using the library
//!
//! ## Display
//!
//! A [`Display`] instance controls the LEDs and programs a timer. There will
//! normally be a single `Display` instance in a program using this library.
//!
//! `Display` is generic over a type implementing [`Frame`], which in turn
//! determines the [`Matrix`] in use.
//!
//! ## Putting it together
//!
//! Once you have provided implementations of all the necessary traits, you
//! can use this library as follows:
//!
//! When your program starts, call [`initialise_control()`] (passing it the
//! device implementing `DisplayControl`) and [`initialise_timer()`] (passing
//! it the device implementing `DisplayTimer`), and create a [`Display`] using
//! your [`Frame`] type.
//!
//! In an interrupt handler for the timer you used for `initialise_timer()`,
//! call [`Display::handle_event()`], passing it the same two devices.
//!
//! To display an image: create a [`Frame`] instance, use [`Frame::set()`] to
//! put the image in it, then call [`Display::set_frame()`].
//!
//! You can call `set_frame()` at any time, so long as you're not
//! interrupting, or interruptable by, `handle_event()`.
//!
//! Once you've called `set_frame()`, you are free to reuse the `Frame`
//! instance.
//!
//!
//! [micropython]: https://microbit-micropython.readthedocs.io/


mod control;
mod display;
mod timer;
mod render;

pub use control::DisplayControl;
pub use display::{RowPlan, Matrix, Frame, Display,
                  initialise_timer, initialise_control,
};
pub use timer::DisplayTimer;
pub use render::{BRIGHTNESSES, MAX_BRIGHTNESS, Render};
