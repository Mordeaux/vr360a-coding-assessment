

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
