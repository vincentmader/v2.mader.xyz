import numpy as np
# from scipy import imageio
import imageio
from tqdm import tqdm
from PIL import Image


img = np.array(imageio.imread('./bachelor_thesis.png'))

new_img = []
for row in tqdm(img):
    new_row = []
    for col in row:
        new_col = np.array(4*[0])
        for px_val in col:
            if px_val != 255:
                new_col = col
                break
        new_row.append(new_col)
    new_img.append(new_row)

new_img = np.array(new_img)
im = Image.fromarray(new_img)
im.save("your_file.jpeg")
print('a')
print(type(new_img))
print(new_img)
