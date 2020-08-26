import gql from 'graphql-tag';

export const GET_COURSES = gql`
  query GetCourses($filter: String!) {
    courses(filter: $filter) {
      id
      title
      teacher_address
      students
    }
  }
`;

export const GET_COURSE_INFO = gql`
  query GetCourseInfo($courseId: String!) {
    myAddress
    course(courseId: $courseId) {
      id
      title
      students
      teacher_address
      sections {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const CREATE_COURSE = gql`
  mutation CreateCourse($title: String!) {
    createCourse(title: $title) {
      id
      title
      teacher_address
    }
  }
`;

export const DELETE_COURSE = gql`
  mutation DeleteCourse($courseId: ID!) {
    deleteCourse(courseId: $courseId) {
      courses(filter: "get_all_courses") {
        id
        title
        teacher_address
        students
      }
    }
  }
`;

export const DELETE_SECTION = gql`
  mutation DeleteSection($courseId: ID!, $sectionId: ID!) {
    deleteSection(courseId: $courseId, sectionId: $sectionId) {
      id
      title
      students
      teacher_address
      sections {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const DELETE_CONTENT = gql`
  mutation DeleteContent($courseId: ID!, $contentId: ID!) {
    deleteContent(courseId: $courseId, contentId: $contentId) {
      id
      title
      students
      teacher_address
      sections {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const UPDATE_CONTENT = gql`
  mutation UpdateContent(
    $courseId: ID!
    $contentId: ID
    $content: ContentInput!
  ) {
    updateContent(courseId: $courseId, contentId: $contentId, content: $content) {
      id
      title
      students
      teacher_address
      sections {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const CREATE_SECTION = gql`
  mutation CreateSection($courseId: ID!, $title: String!) {
    createSection(courseId: $courseId, title: $title) {
      id
      title
      students
      teacher_address
      sections {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const UPDATE_SECTION = gql`
  mutation UpdateSection($courseId: ID!, $sectionId: ID!, $title: String!) {
    updateSection(courseId: $courseId, sectionId: $sectionId, title: $title) {
      id
      title
      students
      teacher_address
      sections {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const CREATE_CONTENT = gql`
  mutation CreateContent(
    $courseId: ID!
    $sectionId: ID!
    $content: ContentInput!
  ) {
    createContent(courseId: $courseId, sectionId: $sectionId, content: $content) {
      id
      title
      students
      teacher_address
      sections {
        id
        title
        contents {
          id
          name
          description
          url
        }
      }
    }
  }
`;

export const ENROL_IN_COURSE = gql`
  mutation EnrolInCourse($courseId: ID!) {
    enrolInCourse(courseId: $courseId) {
      id
      title
      students
    }
  }
`;
