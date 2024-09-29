This is an app I created for a coding assessment as part of an interview process

# Setup

The first time you run this you will need to initialize the database
and run the migrations:

```bash
docker compose run backend diesel setup
docker compose run backend diesel migration run
```

Then you can run the system with:

```bash
docker compose up --build
```

Note that the host and client executables are compiled at build time,
and thus they do not automatically update when you develop. You will have
to build again if you update their code for some reason.


# Architecture

## data_collector

This Rust bin contains the code needed to run both host mode and client mode.
Whether a particular instance is running in host mode or client mode is determined
by the `DATA_COLLECTOR_HOST_MODE` environment variable. If this is set to `1`, then
it will run in host mode. All other settings, including unset, will result in
client mode.

The client connects to the host via websockets. The host can be set with the
`DATA_COLLECTOR_HOST` environment variable.

The host simply posts all of the data it receives to the web app backend.

## backend

A rocket app. Pretty simple.

## frontend

Vite

# Things I ran out of time for

* A websocket connection between frontend and backend that allows realtime
updating of info
* A UI framework
* A more interesting UI in general, a graph would have been nice
* Some kind of authentication, other security measures like encryption
* Better handline of environment stuff to make deploying easier
* TESTS! There are no tests. Luckily the simplicity of the app combined with
the power of Rust and Typescript rules out a lot of potential bugs.
* I've left some `unwraps()` and things of that nature around that need better
handling
* The code could be organized a bit better
