import { SchemaDirectiveVisitor } from 'graphql-tools';

import { INSTANCE_NAME, ZOME_NAME } from '../config';
import { parseEntry, parseResponse } from '../utils';

export class LoadEntityDirective extends SchemaDirectiveVisitor {
  visitFieldDefinition(field) {
    const { entryType } = this.args;
    let defaultResolver = field.resolve;

    field.resolve = async (parent, args, context, info) => {
      let entityId;

      if (defaultResolver) {
        entityId = await defaultResolver(parent, args, context, info);
      } else if (args.courseId) {
        entityId = args.courseId;
      } else {
        entityId = parent[field.name];
      }

      if (!entityId) return null;

      if (typeof entityId === 'string')
        return this.loadEntry(entityId, context.callZome, entryType);
      else return entityId.map(id => this.loadEntry(id, context.callZome, entryType));
    };
  }

  async loadEntry(entityId, callZome, entryType) {

    const zomeFn =
      entryType === "course"
      ? "get_latest_course_entry"
      : entryType === "section"
      ? "get_latest_section_entry"
      : "get_entry";

    const zomeArgs = 
      entryType === "course"
      ? {course_anchor_address : entityId}
      : entryType === "section"
      ? {section_anchor_address: entityId}
      : {address: entityId};

    const entryResult = await callZome(
      INSTANCE_NAME,
      ZOME_NAME,
      zomeFn,
    )(zomeArgs);

    const entry =
    entryType === "course" || entryType === "section"  ? 
    parseResponse(entryResult)
    : parseEntry(entryResult);

    return { id: entityId, ...entry };
  }
}
