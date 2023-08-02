import requests

def execute_curl():
    # Read content from files
    with open("code", "r") as code_file:
        code_content = code_file.read()

    with open("input", "r") as input_file:
        input_content = input_file.read()

    # Prepare the JSON data with the code and input content
    json_data = {
        "language": "python",
        "code": code_content,
        "input": input_content
    }

    # Send the HTTP POST request using requests
    response = requests.post("http://localhost:6070/api/compile", json=json_data)

    # Check if the request was successful
    if response.status_code == 200:
        print(f"{response.text}", end='')

if __name__ == "__main__":
    execute_curl()
