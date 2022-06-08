import rust_ext
import numpy as np


# print(image_augmentation.sum_as_string(5, 20))
# import os
# from PIL import Image

# IMG_DIR = "images"
# images_filename = os.listdir(IMG_DIR)
# images = []
# for image_name in images_filename:
#     img = np.asarray(Image.open(os.path.join(IMG_DIR, image_name)))
#     images.append(img)

# print(len(images))

f = np.array([0.0, 1.0])
rust_ext.mult(15.0, f)
print(f)
