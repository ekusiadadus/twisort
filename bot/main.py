import requests
import os
import urllib
import json
import datetime
from google.cloud import bigquery

bearer_token = os.environ.get("BEARER_TOKEN")
headers = {"Authorization": "Bearer {}".format(bearer_token)}


def create_url():
    now = datetime.datetime.now(
        datetime.timezone.utc) - datetime.timedelta(minutes=3)
    start = now - datetime.timedelta(days=1)
    end = now
    start = '{:%Y-%m-%dT%H:%M:%SZ}'.format(start)
    end = '{:%Y-%m-%dT%H:%M:%SZ}'.format(end)
    # remove retweet
    query = urllib.parse.quote("Go言語")
    tweet_fields = "tweet.fields=author_id,created_at,entities,geo,id,in_reply_to_user_id,lang,possibly_sensitive,referenced_tweets,source,text,withheld"
    url = "https://api.twitter.com/2/tweets/search/recent?query={}+-is%3Aretweet&{}&start_time={}&end_time={}".format(
        query, tweet_fields, start, end)
    return url


def get_params():
    return {"max_results": 10}


def connect_to_endpoint(url, params):
    response = requests.request("GET", url, headers=headers, params=params)
    print(response.status_code)
    if response.status_code != 200:
        raise Exception(response.status_code, response.text)
    return response.json()


def convert_json_to_ndjson():
    with open("data.json", "r") as f:
        data = json.load(f)
    with open("data.ndjson", "w") as f:
        for tweet in data["data"]:
            f.write(json.dumps(tweet))
            f.write("\n")


def insert_into_bigquery():
    client = bigquery.Client()
    dataset_id = 'twitter'

    dataset_ref = client.dataset(dataset_id)
    job_config = bigquery.LoadJobConfig()
    job_config.autodetect = True
    job_config.schema_update_options = [
        bigquery.SchemaUpdateOption.ALLOW_FIELD_ADDITION,
        bigquery.SchemaUpdateOption.ALLOW_FIELD_RELAXATION
    ]
    job_config.source_format = bigquery.SourceFormat.NEWLINE_DELIMITED_JSON
    # data.ndjson

    with open('data.ndjson', 'rb') as source_file:
        job = client.load_table_from_file(
            source_file, dataset_ref.table("test"), job_config=job_config)

    print("Starting job {}".format(job.job_id))

    job.result()  # Waits for table load to complete.
    print("Job finished.")

    destination_table = client.get_table(dataset_ref.table("test"))
    print("Loaded {} rows.".format(destination_table.num_rows))


def main():
    url = create_url()
    params = get_params()
    json_response = connect_to_endpoint(url, params)
    with open('data.json', 'w') as outfile:
        json.dump(json_response, outfile)
    convert_json_to_ndjson()
    insert_into_bigquery()


if __name__ == "__main__":
    main()
