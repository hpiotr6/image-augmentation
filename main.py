# print(image_augmentation.sum_as_string(5, 20))
import os

import matplotlib.pyplot as plt
import numpy as np
import rust_ext
from PIL import Image

IMG_DIR = "images"
images_filename = os.listdir(IMG_DIR)
images = []
for idx, image_name in enumerate(images_filename):
    img = np.asarray(
        Image.open(os.path.join(IMG_DIR, image_name)).convert("RGB")
    ).astype("uint8")
    images.append(img)
    print(img.shape)
    if idx == 1:
        break

images = np.asarray(images)
print("=" * 50)

print(images[0][0])
print(images[1][0])
print("=" * 50)

# print(images.shape)

print(images[0][0])
print(images[1][0])
# print(abc.shape)

# f = np.array([0.0, 1.0])
# rust_ext.mult(15.0, f)
# print(f)


plt.figure(1)
plt.imshow(images[0])
rust_ext.process(images)
plt.figure(2)
plt.imshow(images[0])
plt.show()
