<p align="center">
  <h3 align="center">
      anticine-db
  </h3>
  <p align="center">
    Self-hosted key/value database for <a src="https://github.com/GNUfamilia-fisi/anticine">anticine</a>
  </p>
</p>

<img src="https://raw.githubusercontent.com/GNUfamilia-fisi/anticine/main/media/Anticine.png" />

## Usage

Inicia el servidor TCP de la base de datos:

```bash
$ cargo run
listening for connections at 127.0.0.1:7878
```

Conecta un cliente y empieza a usar queries:

```bash
$ node src/REPLclient.mjs
> SET anticine "is awesome"
> GET anticine
"is awesome"
```

## Queries

- `SET <key> <value>`

- `GET <key>`

> **Warning**
> No listo para producción
