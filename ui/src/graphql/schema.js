import gql from 'graphql-tag';

export const typeDefs = gql`
  directive @loadEntry(entryType: String) on FIELD_DEFINITION

  type Course {
    id: ID!
    title: String!
    sections: [Section!]! @loadEntry(entryType: "section")
    teacher_address: ID!
    students: [ID!]!
  }

  type Section {
    id: ID!
    course_address: Course! @loadEntry(entryType: "course")
    title: String!
    contents: [Content!]! @loadEntry(entryType: "content")
  }

  type Content {
    id: ID!
    name: String!
    description: String!
    url: String!
  }

  type Query {
    courses(filter: String!): [Course!]! @loadEntry(entryType: "course")
    course(courseId: ID!): Course! @loadEntry(entryType: "course")
    myAddress: ID!
  }

  input ContentInput {
    name: String!
    description: String!
    url: String!
  }

  type Mutation {
    createCourse(title: String!): Course! @loadEntry(entryType: "course")
    updateCourse(courseId: ID!, title: String!, sectionsIds: [ID!]!): Course!
      @loadEntry(entryType: "course")
    deleteCourse(courseId: ID!): Query
    enrolInCourse(courseId: ID!): Course! @loadEntry(entryType: "course")
  }
`;

/*
Once you are done with the homework, add the following inside the type Mutation
createSection(courseId: ID!, title: String!): Course! @loadEntry(entryType: "course")
updateSection(courseId: ID!, sectionId: ID!, title: String!): Course!
  @loadEntry(entryType: "course")
deleteSection(courseId: ID!, sectionId: ID!): Course! @loadEntry(entryType: "course")
createContent(courseId: ID!, sectionId: ID!, content: ContentInput!): Course! @loadEntry(entryType: "course")
updateContent(courseId: ID!, contentId: ID!, content: ContentInput!): Course! @loadEntry(entryType: "course")
deleteContent(courseId: ID!, contentId: ID!): Course! @loadEntry(entryType: "course")
 */
