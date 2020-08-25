import { INSTANCE_NAME, ZOME_NAME } from '../config';
import { parseResponse } from '../utils';

export const resolvers = {
  Query: {
    async courses(_, { filter }, { callZome }) {
      const fnName =
        filter === 'enrolled-courses'
          ? 'get_my_enrolled_courses'
          : filter === 'my-courses'
          ? 'get_my_courses'
          : 'get_all_courses';

      const result = await callZome(INSTANCE_NAME, ZOME_NAME, fnName)({});

      return parseResponse(result);
    },
    async myAddress(_, __, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'get_my_address'
      )({});

      return parseResponse(result);
    }
  },
  Course: {
    async students(parent, _, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'get_all_students'
      )({
        course_anchor_address: parent.id
      });

      return parseResponse(result);
    }
  },
  Section: {
    async contents(parent, _, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'get_contents'
      )({
        section_anchor_address: parent.id
      });

      return parseResponse(result);
    }
  },
  Mutation: {
    async createCourse(_, { title }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'create_course'
      )({
        title,
        timestamp: getTimestamp()
      });

      return parseResponse(result);
    },
    async updateCourse(_, { title, courseId, sectionsAddresses }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'update_course'
      )({
        title,
        course_anchor_address: courseId,
        sections_addresses: sectionsAddresses,
        timestamp: getTimestamp()
      });

      return parseResponse(result);
    },
    async deleteCourse(_, { courseId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'delete_course'
      )({
        course_anchor_address: courseId
      });

      return parseResponse(result);
    },
    // HOMEWORK
    // async createSection(_, { courseId, title }, { callZome }) {

    // },
    // HOMEWORK
    // async updateSection(_, { courseId, sectionId, title }, { callZome }) {

    // },
    // HOMEWORK
    // async deleteSection(_, { courseId, sectionId }, { callZome }) {

    // },
    // HOMEWORK
    // async createContent(_, { courseId, content, sectionId }, { callZome }) {

    // },
    // HOMEWORK
    // async updateContent(_, { courseId, content, contentId }, { callZome }) {

    // },
    // HOMEWORK
    // async deleteContent(_, { courseId, contentId }, { callZome }) {

    // },
    async enrolInCourse(_, { courseId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'enrol_in_course'
      )({
        course_anchor_address: courseId
      });

      parseResponse(result);
      return courseId;
    }
  }
};

function getTimestamp() {
  return Math.floor(Date.now() / 1000);
}
