import requests
import unittest
import json


class Plugin(unittest.TestCase):
    OK = 200
    BAD_REQUEST = 400
    INTERNAL_SERVER = 500

    HOST = 'http://localhost:8080'
    TOKEN = ".tokens.json"

    def post_json(self, uri, token=None, params=None, status=200):
        uri = self.api(uri)
        print('HTTP POST {uri}'.format(uri=uri))
        print(json.dumps(params))
        res = requests.post(uri, json=params)
        print(res.text)
        self.assertEqual(res.status_code, status)
        if status == self.OK:
            return res.json()

    def get_json(self, uri, token=None, status=200):
        print('HTTP GET {uri}'.format(uri=uri))
        res = requests.get(self.api(uri))
        self.assertEqual(res.status_code, status)
        if status == self.OK:
            return res.json()
        print(res.text)

    def html(self, uri, status=200):
        uri = self.HOST + uri
        print('HTTP GET {uri}'.format(uri=uri))
        res = requests.get(uri)
        print(res.text)
        self.assertEqual(status, res.status_code)

    def api(self, uri):
        return self.HOST + '/api' + uri
