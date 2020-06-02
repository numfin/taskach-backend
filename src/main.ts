import { createApp } from "lib/createApp";
import { UserModule } from "src/user/users.module.ts";

const { server } = createApp({
  modules: [UserModule],
});

server.listen(8080);
