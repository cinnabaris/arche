import TopicsIndex from './topics/Index'
import TopicsForm from './topics/Form'
import PostsIndex from './posts/Index'
import PostsForm from './posts/Form'
import TagsIndex from './tags/Index'
import TagsForm from './tags/Form'

export default[
  {
    name: 'forum.topics.index',
    path: '/forum/topics',
    component: TopicsIndex
  }, {
    name: 'forum.topics.new',
    path: '/forum/topics/new',
    component: TopicsForm
  }, {
    name: 'forum.topics.edit',
    path: '/forum/topics/:id/edit',
    component: TopicsForm
  }, {
    name: 'forum.tags.index',
    path: '/forum/tags',
    component: TagsIndex
  }, {
    name: 'forum.tags.new',
    path: '/forum/tags/new',
    component: TagsForm
  }, {
    name: 'forum.tags.edit',
    path: '/forum/tags/:id/edit',
    component: TagsForm
  }, {
    name: 'forum.posts.index',
    path: '/forum/posts',
    component: PostsIndex
  }, {
    name: 'forum.posts.new',
    path: '/forum/topics/:topic/post',
    component: PostsForm
  }, {
    name: 'forum.posts.edit',
    path: '/forum/posts/:id/edit',
    component: PostsForm
  }
]
