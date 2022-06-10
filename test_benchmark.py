import os
import numpy as np
import pytest
import rust_ext
from PIL import Image


@pytest.fixture(scope="session")
def contents() -> str:
    IMG_DIR = "images"
    images_filename = os.listdir(IMG_DIR)
    images = []
    for image_name in images_filename:
        img = np.asarray(
            Image.open(os.path.join(IMG_DIR, image_name)).convert("RGB")
        ).astype("uint8")
        images.append(img)

    images = np.asarray(images)
    return images


def test_rust_parallel(benchmark, contents):
    benchmark(rust_ext.process_parallel, contents)


def test_rust_seq(benchmark, contents):
    benchmark(rust_ext.process_sequential, contents)
