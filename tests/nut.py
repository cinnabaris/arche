

from .plugin import Plugin


class TestNut(Plugin):
    def test_install(self):
        params = {}
        # username
        self.post_json('/install', params=params, status=self.BAD_REQUEST)
        params['name'] = ''
        self.post_json('/install', params=params, status=self.BAD_REQUEST)
        params['name'] = 'aaa'
        self.post_json('/install', params=params, status=self.BAD_REQUEST)
        # email
        params['email'] = 'aaa'
        self.post_json('/install', params=params, status=self.BAD_REQUEST)
        params['email'] = 'aaa@aaa.com'
        self.post_json('/install', params=params, status=self.BAD_REQUEST)
        # password
        params['password'] = '12345'
        self.post_json('/install', params=params, status=self.INTERNAL_SERVER)
        # should ok
        params['password'] = '123456'
        self.post_json('/install', params=params, status=self.OK)
        # fail
        self.post_json('/install', params=params, status=self.INTERNAL_SERVER)

    def test_layout(self):
        self.get_json('/layout')

    def test_locales(self):
        for l in ['en-US', 'zh-Hans', 'zh-Hant']:
            self.get_json('/locales/' + l)

    def test_index(self):
        self.html('/')
