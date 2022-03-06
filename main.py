import yaml
import sys
import subprocess


def main():
    subprocess.check_call([sys.executable, '-m', 'pip', 'install',
                           'pyyaml'])
    i = 0
    plexFile = open('indefbans.yml', 'a')
    path = input("Enter the path to the TotalFreedomMod indefinitebans.yml file: ")
    with open(path, 'r') as tfmFile:
        indefiniteBans = yaml.safe_load(tfmFile)
        plexFile.writelines("# Plex Indefinite Bans File\n")
        plexFile.writelines(
            "# Players with their UUID / IP / Usernames in here will be indefinitely banned until removed\n")
        plexFile.writelines("\n")
        plexFile.writelines(
            "# If you want to get someone's UUID, use https://api.ashcon.app/mojang/v2/user/<username>\n")
        for item, doc in indefiniteBans.items():
            dictionary = dict(doc)
            plexFile.writelines(f'{i}:\n')
            plexFile.writelines('  users:\n')
            plexFile.writelines(f'  - {item}\n')
            print(f"Added username: {item}")
            if dictionary.__contains__('uuid'):
                plexFile.writelines(f'  uuids:\n')
                plexFile.writelines(f"  - {dictionary.get('uuid')}\n")
                print(f"Added UUID: {dictionary.get('uuid')}")
            # Make sure it has an IP, make sure the IP isn't: [], and make sure the ip: isn't empty
            if dictionary.__contains__('ips') and dictionary.get('ips') != "[]" and dictionary.get('ips'):
                plexFile.writelines(f'  ips:\n')
                for ips in dictionary.get("ips"):
                    plexFile.writelines(f"  - {ips}\n")
                    print(f"Added IP: {ips}")
            i += 1
    tfmFile.close()
    plexFile.close()

    print("-----------------------------------------------------------------------------------------------")
    print("Converted TotalFreedomMod indefinitebans.yml to Plex's format!")
    print("Your new file has been saved as 'indefbans.yml' in the same directory the script was ran in.")
    print("The final step is to upload this to the Plex plugin folder.")
    print("-----------------------------------------------------------------------------------------------")


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    main()
