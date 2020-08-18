/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

// for less verbose test TRYORAMA_LOG_LEVEL=error hc test


const path = require("path");

// we import necessary things here from tryorama.
const {Orchestrator, Config} = require("@holochain/tryorama");

// here, we are defining the path to the course dna 
const dnaPath = path.join(__dirname, "../dist/course.dna.json");

// we are instantiating an orchestrator object here.
// Test suites are defined with an Orchestrator object. 
// This default orchestrator is configured to use tapeExecutor and localOnly middleware.
const orchestrator = new Orchestrator();

// Point to your DNA file and give it a nickname.
const dna = Config.dna(dnaPath, "course_dna");

// Config.gen is a handy shortcut for creating a full-fledged conductor config
// from as little information as possible
const conductorConfig = Config.gen(
  { course_dna: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    }
  }
);

// These are HOF(higher order functions) for zomeFn calls
// that will make our codes more readable
function createCourse(title, timestamp) {
  return (caller) => 
    caller.call("course_dna", "courses", "create_course", {
      title,
      timestamp,
    })
};

function updateCourse(title, sectionsAddresses, courseAnchorAddress, timestamp) {
  return (caller) =>
  caller.call("course_dna", "courses", "update_course", {
    title,
    sections_addresses: sectionsAddresses,
    course_anchor_address: courseAnchorAddress,
    timestamp,
  }) 
};

function deleteCourse(courseAnchorAddress) {
  return (caller) =>
    caller.call("course_dna", "courses", "delete_course", {
      course_anchor_address: courseAnchorAddress,
    })
};

function getLatestCouseEntry(courseAnchorAddress) {
  return (caller) =>
    caller.call("course_dna", "courses", "get_latest_course_entry", {
      course_anchor_address: courseAnchorAddress,
    })
};

function getMyCourses() {
  return (caller) =>
    caller.call("course_dna", "courses", "get_my_courses", {})
};

function enrolInCourse(courseAnchorAddress) {
  return (caller) =>
    caller.call("course_dna", "courses", "enrol_in_course", {
      course_anchor_address: courseAnchorAddress,
    })
};

function getMyEnrolledCourse() {
  return (caller) =>
    caller.call("course_dna", "courses", "get_my_enrolled_courses", {})
};

function getAllStudents(courseAnchorAddress) {
  return (caller) =>
    caller.call("course_dna", "courses", "get_all_students", {
      course_anchor_address: courseAnchorAddress,
    })
};

function createSection(title, courseAnchorAddress, timestamp) {
  return (caller) =>
  caller.call("course_dna", "courses", "create_section", {
    title,
    course_anchor_address: courseAnchorAddress,
    timestamp,
  })
};

function updateSection(title, sectionAnchorAddress, timestamp) {
  return (caller) =>
  caller.call("course_dna", "courses", "update_section", {
    title,
    section_anchor_address: sectionAnchorAddress,
    timestamp,
  })
};

function deleteSection(sectionAnchorAddress, timestamp) {
  return (caller) =>
    caller.call("course_dna", "courses", "delete_section", {
      section_anchor_address: sectionAnchorAddress,
      timestamp,
    })
};

function getLatestSectionEntry(sectionAnchorAddress) {
  return (caller) =>
    caller.call("course_dna", "courses", "get_latest_section_entry", {
      section_anchor_address: sectionAnchorAddress,
    })
};

function createContent(name, sectionAnchorAddress, url, timestamp, description) {
  return (caller) =>
  caller.call("course_dna", "courses", "create_content", {
    name,
    section_anchor_address: sectionAnchorAddress,
    url,
    timestamp,
    description,
  })
};

function updateContent(contentAddress, name, url, description, timestamp) {
  return (caller) =>
  caller.call("course_dna", "courses", "update_content", {
    content_address: contentAddress,
    name,
    url,
    description,
    timestamp,
  })
};

function deleteContent(contentAddress) {
  return (caller) =>
    caller.call("course_dna", "courses", "delete_content", {
      content_address: contentAddress,
    })
};

function getContents(sectionAnchorAddress) {
  return (caller) =>
    caller.call("course_dna", "courses", "get_contents", {
      section_anchor_address: sectionAnchorAddress,
    })
};


/*******  CREATE_COURSE & GET_LATEST_COURSE_ENTRY *********/
// here we are registering the first test scenario through orchestrator object.
// registerScenario accepts two arguments. The first arg is the name of your test scenario.
// The second argument is the actual test that is in a form of async fnction.
// `s` is an instance of the Scenario API
// `t` is the tape assertion API
// In this scenario, we are testing the create_course and get_latest_course_entry function
orchestrator.registerScenario("Scenario1: Create new course and get latest", async (s, t) => {

  // here, we are saying that there are 2 agents in this test scenario
  // both of which we are giving the same conductor config we defined above in L26.
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );

  // so first we are making alice to create a course 
  const course_addr = await createCourse("course test 1", 123)(alice);
  console.log(course_addr);

  // here we are checking if we are asserting 
  // that value is truthy
  t.ok(course_addr.Ok);

  // another use of the Scenario API is to automagically wait
  // for the network to reach a consistent state
  // before continuing the test
  await s.consistency();

  // Now, bob is trying to get the course entry that was committed by alice
  const course = await getLatestCouseEntry(course_addr.Ok)(bob);
  console.log("course");
  console.log(course);

  // this is a strict comparison (===) between
  // course.Ok and the object in second argument
  t.deepEqual(course.Ok, {
    title: "course test 1",
    timestamp: 123,
    teacher_address: alice.instance("course_dna").agentAddress,
    sections: [],
    anchor_address: course_addr.Ok,
  });
  // Wait for all network activity to settle
  await s.consistency();
});

/*********** UPDATE_COURSE ********/
orchestrator.registerScenario("Scenario2: Update course title", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await createCourse("new course test for update test", 123)(alice);
  const course_update_addrss = await updateCourse("course title updated", [], course_addr.Ok, 1234)(alice);
  await s.consistency();
  const course = await getLatestCouseEntry(course_addr.Ok)(bob);
  t.deepEqual(course.Ok, {
    title: "course title updated",
    timestamp: 1234,
    teacher_address: alice.instance("course_dna").agentAddress,
    sections: [],
    anchor_address: course_update_addrss.Ok
  });
  const course_update_addrss_2 = await updateCourse("new course test for update test", [], course_addr.Ok, 12345)(alice);
  await s.consistency();
  const course2 = await getLatestCouseEntry(course_update_addrss_2.Ok)(bob);
  t.deepEqual(course2.Ok, {
    title: "new course test for update test",
    timestamp: 12345,
    teacher_address: alice.instance("course_dna").agentAddress,
    sections: [],
    anchor_address: course_update_addrss.Ok
  });
});

/*********** DELETE_COURSE ********/
orchestrator.registerScenario("Scenario3: Delete course", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await createCourse("new course test for delete scenario", 123)(alice);
  await s.consistency();

  const delete_result = await deleteCourse(course_addr.Ok)(alice);
  await s.consistency();
  
  console.log("deleted");
  console.log(delete_result);
  t.ok(delete_result.Ok);

  const course = await getLatestCouseEntry(course_addr.Ok)(bob);
  console.log("course");
  console.log(course);
  t.deepEqual(course.Ok, null);
  await s.consistency();

});

/********* GET_MY_COURSES *********/
orchestrator.registerScenario("Scenario4: Get All My Courses", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr_1 = await createCourse("course for scenario 5-1", 123)(alice);
  console.log(course_addr_1);
  t.ok(course_addr_1.Ok);

  await s.consistency();

  const course_addr_2 = await createCourse("course for scenario 5-2", 1234)(alice);
  console.log(course_addr_2);
  t.ok(course_addr_2.Ok);

  await s.consistency();


  const all_courses_alice = await getMyCourses()(alice);
  t.true(all_courses_alice.Ok[0] != null);
  t.true(all_courses_alice.Ok[1] != null);

  const all_courses_bob = await getMyCourses()(bob);
  t.true(all_courses_bob.Ok[0] == null);

  await s.consistency();
});

/********** ENROL_IN_COURSE & GET_ALL_STUDENTS & GET_MY_ENROLLED_COURSES **********/
orchestrator.registerScenario("Scenario5: Create new course and enrol", async (s, t) => {
  const { alice, bob } = await s.players(
    { alice: conductorConfig, bob: conductorConfig },
    true
  );
  const course_addr = await createCourse("course test 1", 123)(alice);
  console.log(course_addr);
  t.ok(course_addr.Ok);

  await s.consistency();

  const enrolled_addr = await enrolInCourse(course_addr.Ok)(bob);
  console.log("enrolled");
  console.log(enrolled_addr);
  t.ok(enrolled_addr.Ok);

  await s.consistency();

  const all_enrolled_courses = await getMyEnrolledCourse()(bob);
  console.log("my_enrolled_courses");
  console.log(all_enrolled_courses);
  t.true(all_enrolled_courses.Ok[0] != null);

  const all_students = await getAllStudents(course_addr.Ok)(alice);
  console.log("all_students");
  console.log(all_students);
  t.true(all_students.Ok[0] != null);


  // Wait for all network activity to settle
  await s.consistency();
});

// HOMEWORK
/******** CREATE_SECTION & GET_LATEST_SECTION_ENTRY **********/
// orchestrator.registerScenario("Scenario6: Create new Section for a Course", async (s, t) => {
  
// });

// HOMEWORK
/********* CREATE_CONTENT ********/
// orchestrator.registerScenario("Scenario7: Create new Content for a Section", async (s, t) => {

// });

// HOMEWORK
/********* GET_CONTENTS *******/
// orchestrator.registerScenario("Scenario8: Get all contents of a module", async (s, t) => {

// });

// HOMEWORK
/********* DELETE_CONTENT & GET_CONTENTS **********/
// orchestrator.registerScenario("Scenario9: delete content from module", async (s, t) => {
  
// });

// HOMEWORK
/***** DELETE_SECTION ***********/
// orchestrator.registerScenario("Scenario10: delete module from course", async (s, t) => {

// });

// HOMEWORK
/********** UPDATE_SECTION & UPDATE_CONTENT ********/
// orchestrator.registerScenario("Scenario11: update section and content", async (s, t) => {

// });

orchestrator.run();