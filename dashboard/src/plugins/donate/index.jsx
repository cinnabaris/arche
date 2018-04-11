import {USER, ADMIN} from '../../auth'

const ProjectsForm = import ('./projects/Form')

export default {
  routes: [
    {
      path: "/donate/projects/edit/:id",
      component: ProjectsForm
    }, {
      path: "/donate/projects/new",
      component: ProjectsForm
    }, {
      path: "/donate/projects",
      component: import ('./projects/Index')
    }
  ],
  menus: [
    {
      icon: "pay-circle-o",
      label: "donate.dashboard.title",
      href: "donate",
      roles: [
        USER, ADMIN
      ],
      items: [
        {
          label: "donate.projects.index.title",
          href: "/donate/projects"
        }
      ]
    }
  ]
}
