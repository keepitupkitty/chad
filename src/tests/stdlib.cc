#include <gtest/gtest.h>
#include <gmock/gmock.h>

#include <climits>

extern "C" {
  void *ouma_aligned_alloc(size_t alignment, size_t size);
  void *ouma_calloc(size_t nmemb, size_t size);
  void ouma_free(void *ptr);
  void ouma_free_sized(void *ptr, size_t size);
  void ouma_free_aligned_sized(void *ptr, size_t alignment, size_t size);
  void *ouma_malloc(size_t size);
  void *ouma_realloc(void *ptr, size_t size);
  int ouma_posix_memalign(void **memptr, size_t alignment, size_t size);
  unsigned long ouma_strtoul(const char *src, char **endptr, int base);

  extern _Thread_local int __oumalibc_errno;
}

TEST(malloc, zero) {
  void *b1 = ouma_malloc(0);
  void *b2 = ouma_malloc(0);
  ASSERT_TRUE((b1 == NULL && b2 == NULL) ||
              (b1 != NULL && b2 != NULL && b1 != b2));
  ouma_free(b1);
  ouma_free(b2);
}

TEST(malloc, simple) {
  void *ptr = ouma_malloc(100);
  ASSERT_TRUE(ptr != nullptr);
  ouma_free(ptr);
}

TEST(malloc, overflow) {
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_malloc(SIZE_MAX));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
}

TEST(malloc, malloc_realloc_larger) {
  char *ptr = (char *)ouma_malloc(100);
  ASSERT_TRUE(ptr != nullptr);
  memset(ptr, 67, 100);
  ptr = (char *)ouma_realloc(ptr, 200);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 100; i++) {
    ASSERT_EQ(67, ptr[i]);
  }
  ouma_free(ptr);
}

TEST(malloc, malloc_realloc_smaller) {
  char *ptr = (char *)ouma_malloc(200);
  ASSERT_TRUE(ptr != nullptr);
  memset(ptr, 67, 200);
  ptr = (char *)ouma_realloc(ptr, 100);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 100; i++) {
    ASSERT_EQ(67, ptr[i]);
  }
  ouma_free(ptr);
}

TEST(malloc, malloc_multiple_realloc) {
  char *ptr = (char *)ouma_malloc(200);
  ASSERT_TRUE(ptr != nullptr);
  memset(ptr, 0x23, 200);
  ptr = (char *)ouma_realloc(ptr, 100);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 100; i++) {
    ASSERT_EQ(0x23, ptr[i]);
  }
  ptr = (char*)ouma_realloc(ptr, 50);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 50; i++) {
    ASSERT_EQ(0x23, ptr[i]);
  }
  ptr = (char*)ouma_realloc(ptr, 150);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 50; i++) {
    ASSERT_EQ(0x23, ptr[i]);
  }
  memset(ptr, 0x23, 150);
  ptr = (char*)ouma_realloc(ptr, 425);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 150; i++) {
    ASSERT_EQ(0x23, ptr[i]);
  }
  ouma_free(ptr);
}

TEST(malloc, calloc_realloc_larger) {
  char *ptr = (char *)ouma_calloc(1, 100);
  ASSERT_TRUE(ptr != nullptr);
  ptr = (char *)ouma_realloc(ptr, 200);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 100; i++) {
    ASSERT_EQ(0, ptr[i]);
  }
  ouma_free(ptr);
}

TEST(malloc, calloc_realloc_smaller) {
  char *ptr = (char *)ouma_calloc(1, 200);
  ASSERT_TRUE(ptr != nullptr);
  ptr = (char *)ouma_realloc(ptr, 100);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 100; i++) {
    ASSERT_EQ(0, ptr[i]);
  }
  ouma_free(ptr);
}

TEST(malloc, calloc_multiple_realloc) {
  char *ptr = (char *)ouma_calloc(1, 200);
  ASSERT_TRUE(ptr != nullptr);
  ptr = (char *)ouma_realloc(ptr, 100);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 100; i++) {
    ASSERT_EQ(0, ptr[i]);
  }
  ptr = (char*)ouma_realloc(ptr, 50);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 50; i++) {
    ASSERT_EQ(0, ptr[i]);
  }
  ptr = (char*)ouma_realloc(ptr, 150);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 50; i++) {
    ASSERT_EQ(0, ptr[i]);
  }
  memset(ptr, 0, 150);
  ptr = (char*)ouma_realloc(ptr, 425);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < 150; i++) {
    ASSERT_EQ(0, ptr[i]);
  }
  ouma_free(ptr);
}

TEST(malloc, realloc_overflow) {
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_realloc(nullptr, SIZE_MAX));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
  void* ptr = ouma_malloc(100);
  ASSERT_TRUE(ptr != nullptr);
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_realloc(ptr, SIZE_MAX));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
  ouma_free(ptr);
}

TEST(calloc, example) {
  size_t alloc_len = 100;
  char *ptr = (char *)ouma_calloc(1, alloc_len);
  ASSERT_TRUE(ptr != nullptr);
  for (size_t i = 0; i < alloc_len; i++) {
    ASSERT_EQ(0, ptr[i]);
  }
  ouma_free(ptr);
}

TEST(calloc, illegal) {
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_calloc(-1, 100));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
}

TEST(calloc, overflow) {
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_calloc(1, SIZE_MAX));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_calloc(SIZE_MAX, SIZE_MAX));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_calloc(2, SIZE_MAX));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
  __oumalibc_errno = 0;
  ASSERT_EQ(nullptr, ouma_calloc(SIZE_MAX, 2));
  ASSERT_EQ(ENOMEM, __oumalibc_errno);
}

TEST(realloc, null) {
  void *buf = ouma_realloc(NULL, 100);
  ASSERT_NE(nullptr, buf);
  ouma_free(buf);
}

TEST(realloc, example) {
  char *buf = static_cast<char *>(ouma_malloc(64));
  ASSERT_NE(nullptr, buf);
  memset(buf, 'A', 64);

  buf = static_cast<char *>(ouma_realloc(buf, 128));
  ASSERT_NE(nullptr, buf);
  for (size_t i = 0; i < 64; ++i) {
    ASSERT_EQ('A', buf[i]);
  }

  ouma_free(buf);
}

TEST(posix_memalign, bad) {
  for (size_t i = 0; i < sizeof(void *); ++i) {
    ASSERT_EQ(EINVAL, ouma_posix_memalign(nullptr, i, 1));
  }
}

TEST(posix_memalign, example) {
  for (size_t i = sizeof(void *); i < 4096; i *= 2) {
    void *buf;
    ASSERT_EQ(0, ouma_posix_memalign(&buf, i, 1));
    ASSERT_EQ(0, (uintptr_t)buf % i);
    ouma_free(buf);
  }
}

TEST(free, null) {
  ouma_free(nullptr);
}

TEST(free, example) {
  void *buf = ouma_malloc(32);
  ASSERT_NE(nullptr, buf);
  ouma_free(buf);
}
