import os

from .plugin import Plugin


class TestNutApi(Plugin):

    def test_install(self):
        uri = '/install'
        params = {}
        # username
        self.post_json(uri, params=params, status=self.BAD_REQUEST)
        params['name'] = ''
        self.post_json(uri, params=params, status=self.BAD_REQUEST)
        params['name'] = 'aaa'
        self.post_json(uri, params=params, status=self.BAD_REQUEST)
        # email
        params['email'] = 'aaa'
        self.post_json(uri, params=params, status=self.BAD_REQUEST)
        params['email'] = 'aaa@aaa.com'
        self.post_json(uri, params=params, status=self.BAD_REQUEST)
        # password
        params['password'] = '12345'
        self.post_json(uri, params=params, status=self.INTERNAL_SERVER)
        params['password'] = '123456'
        # ok
        if not os.path.exists(self.TOKEN):
            self.post_json(uri, params=params, status=self.OK)
            open(self.TOKEN, 'a').close()
        # fail
        self.post_json(uri, params=params, status=self.INTERNAL_SERVER)

    def test_layout(self):
        self.get_json('/layout')

    def test_locales(self):
        for l in ['en-US', 'zh-Hans', 'zh-Hant']:
            self.get_json('/locales/' + l)


class TestNutHtml(Plugin):
    def test_index(self):
        self.html('/')
