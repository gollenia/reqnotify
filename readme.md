## getRequest

Tiny little rust Program to send HTTP-Requests to any URL with Windows, but _without opening a commandline window_. For me, it simply works as a nodered remote control.

#### The Problem

Especially for smarthome you may sometimes have the need to send a simple Request to a server or another PC or IOT-Device. There is Curl, but I don't want to see a terminal window popping up when darkening the Light in my studio.

#### Solution

So I wrote this tiny program. It sends a request to a server without opening a terminal and notifies you about the result, if needed.

## Building

Head into the root directory and simply type `cargo build` (you must have installed rust first, of course)

## Usage

`C:\[path_to_program]\reqnotify url [-n | --notify] [-v | --version] [-h | --help]`

The request must return a JSON Object containing a title and a text so reqnotify can display the propper info in the toast.

## Future plans

-   Add POST requests
-   Add possibility to add body
-   Add possibility
