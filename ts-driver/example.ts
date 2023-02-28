import { AnticineDB } from "./tsdriver";

!async function() {
  let db = new AnticineDB();

  await db.connect(7878);

  await db.set('hola', { guardando: 'json' });

  let value = await db.get('hola');

  console.log({ value });
}();
