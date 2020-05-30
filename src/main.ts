import { NestFactory } from "@nestjs/core";
import { AppModule } from "./app.module";

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  return app.listen(process.env.PORT || 8080);
}

bootstrap().catch((reason) => {
  console.log(reason);
});
