from pathlib import Path

from aiogram import types
from aiogram.fsm.context import FSMContext
from aiogram.types.input_file import BufferedInputFile
from urlextract import URLExtract
from yt_dlp import YoutubeDL

from to_gaze_bot import states

FORMAT = "mp4"
CACHE_DIR = "./cache"
DOWNLOAD_PARAMS = {"format": FORMAT, "paths": {"home": CACHE_DIR}}


# TODO: Put URLExtract's and YoutubeDL's instatnces into static storage
async def video(msg: types.Message, state: FSMContext) -> None:
    if msg.from_user is None or not msg.text:
        return

    urlextract = URLExtract()
    urls = urlextract.find_urls(text=msg.text, only_unique=True)

    ans = await msg.answer(
        "Видео скачиваeтся..." if len(urls) % 10 == 1 else "Видео скачиваются..."
    )

    # TODO: Add proxy via --proxy socks5://proxy:1080
    downloader = YoutubeDL(DOWNLOAD_PARAMS)

    downloader.download(urls)

    for url in urls:
        url = str(url)

        if "?si" in url:
            video_id = url.split("/")[-1].split("?")[0]
        else:
            video_id = url.split("=")[-1]

        files = Path(CACHE_DIR).glob(f"*{video_id}*")
        for file in files:
            name = file.name.split("[")[:-1]
            video = BufferedInputFile(file.read_bytes(), f"{name}.mp4")
            await msg.answer_video(video)
            await ans.delete()

        await state.set_state(states.user.UserMainMenu.menu)
        return

    await msg.answer("Huuh, что-то пошло не так, попробуй еще")
    await state.set_state(states.user.UserMainMenu.menu)
