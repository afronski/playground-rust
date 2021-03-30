import boto3

client = boto3.client('sts')

response = client.get_session_token(DurationSeconds = 3600)

if 'Credentials' in response:
    creds = response['Credentials']

    file = open('credentials.private', 'w')
    file.write(f'CREDENTIALS {creds["AccessKeyId"]} {creds["Expiration"].isoformat().replace("+00:00", "Z")} {creds["SecretAccessKey"]} {creds["SessionToken"]}\n')
    file.close()
