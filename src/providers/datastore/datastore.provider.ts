import { Datastore, DatastoreOptions } from "@google-cloud/datastore";
import { Injectable } from "@nestjs/common";

@Injectable()
export class DatastoreProvider extends Datastore {
  constructor(settings?: DatastoreOptions) {
    super(settings);
  }

  static forRoot() {
    return new DatastoreProvider();
  }
}
