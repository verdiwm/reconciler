# Reconciler

**Reconciler** is a powerful and composable asynchronous event listener designed
to simplify the management and handling of multiple event streams. It allows you
to efficiently merge multiple event sources into a single stream, which can then
be further composed, manipulated, and handled as needed.

## Overview

Built on top of the robust Tokio runtime, Reconciler leverages the asynchronous
capabilities of Rust to provide a seamless experience for event-driven
applications. Please note that Reconciler is specifically designed to work with
the Tokio runtime and may not be compatible with other runtimes, although most
offer compatibility layers that could be utilized.

## Features

- **Stream Merging:** Reconciler allows you to combine multiple asynchronous
  event streams into a single unified stream, simplifying the complexity of
  handling multiple sources.
- **Flexible Modes of Operation:** The library supports two primary modes of
  operation:
  1. **Event Listener Mode:** Combine multiple event streams into one unified
     stream. This mode is ideal when you need to process events sequentially.
  2. **Event Handler Mode:** Pair each event stream with its respective handler,
     allowing for concurrent and independent processing of events.
- **Composable Design:** Reconciler's composable architecture enables the
  seamless integration and combination of different streams and handlers

## Usage

Reconciler is versatile and can be tailored to fit various use cases. You can
utilize it as an event listener to merge streams or as an event handler to
manage events concurrently. Combining these modes is not only possible but also
encouraged, as it allows for a more robust and scalable event processing system.

### Event Listener Mode

In this mode, Reconciler acts as a central hub that consolidates multiple event
streams into one. Events are processed sequentially, ensuring that each event is
handled in the order it was received.

### Event Handler Mode

In Event Handler mode, each stream is paired with a dedicated handler, enabling
events to be processed concurrently. This mode is particularly useful when
events from different sources need to be handled independently and
simultaneously.

### Combined Usage

By leveraging both modes together, you can create a more complex event-driven
architectures. For example, you can use the Event Listener mode to unify streams
and then dispatch these unified events to different handlers for concurrent
processing.

## Example usage

_TBA_

## License

This project is licensed under the
[Apache-2.0 License](http://www.apache.org/licenses/LICENSE-2.0). For more
information, please see the [LICENSE](LICENSE) file.
