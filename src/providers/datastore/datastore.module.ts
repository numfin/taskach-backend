import { Module } from "@nestjs/common";
import { DatastoreProvider } from "./datastore.provider";

@Module({
  providers: [
    {
      provide: DatastoreProvider,
      useValue: new DatastoreProvider(),
    },
  ],
  exports: [DatastoreProvider],
})
export class DatastoreModule {}
