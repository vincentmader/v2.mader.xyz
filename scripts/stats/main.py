
import os
import pathlib


pwd = pathlib.Path(__file__).parent.resolve()
proj_root = f"{pwd}/../.."


def check_dir(path, length):

    files = os.listdir(path)
    for item in sorted(files):

        if item in ['out', 'target', '.git', 'pkg', 'Cargo.toml']:
            continue
        if item.endswith('.png') or item.endswith('.jpg'):
            continue
        print(item)

        new_path = os.path.join(path, item)

        if os.path.isdir(new_path):
            length = check_dir(new_path, length)

        else:
            with open(new_path, 'rb') as fp:
                content = fp.readlines()
                # print(len(content))
                length += len(content)
                # length += len(''.join(content))
                # print(content)

    return length


length = check_dir(proj_root, 0)
print(length)

