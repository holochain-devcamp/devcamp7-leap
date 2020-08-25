import { LeapDashboard } from './components/leap-dashboard';
import { LeapApp } from './components/leap-app';
import { LeapCoursesList } from './components/leap-courses-list';
import { LeapCourseDetail } from './components/leap-course-detail';
import { LeapSection } from './components/leap-section';
import { LeapEmtpyPlacholder } from './components/leap-emtpy-placeholder';

customElements.define('leap-course-detail', LeapCourseDetail);
customElements.define('leap-courses-list', LeapCoursesList);
customElements.define('leap-app', LeapApp);
customElements.define('leap-dashboard', LeapDashboard);
customElements.define('leap-section', LeapSection);
customElements.define('leap-empty-placeholder', LeapEmtpyPlacholder);
