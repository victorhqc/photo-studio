export async function rebuildApplication() {
  const url = process.env.BUILD_HOOK_URL || 'build-hook-url-not-set';
  await fetch(url, {
    method: 'POST',
  });
}
