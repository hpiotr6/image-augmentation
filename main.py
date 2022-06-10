import os

import matplotlib.pyplot as plt
import numpy as np
import rust_ext
from PIL import Image

IMG_DIR = "images"
images_filename = os.listdir(IMG_DIR)
images = []
for image_name in images_filename:
    img = np.asarray(
        Image.open(os.path.join(IMG_DIR, image_name)).convert("RGB")
    ).astype("uint8")
    images.append(img)


images = np.asarray(images)
print("=" * 50)

plt.figure(1)
plt.imshow(images[0])
rust_ext.process_parallel(images)
plt.figure(2)
plt.imshow(images[0])
plt.show()
