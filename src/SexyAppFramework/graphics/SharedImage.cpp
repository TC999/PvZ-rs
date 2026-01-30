#include "SharedImage.h"
#include "graphics/GLImage.h"
#include "SexyAppBase.h"

using namespace Sexy;

SharedImage::SharedImage() :
	mImage(nullptr),
	mRefCount(0)
{
}

SharedImageRef::SharedImageRef(const SharedImageRef& theSharedImageRef) :
	mSharedImage(theSharedImageRef.mSharedImage),
	mUnsharedImage(theSharedImageRef.mUnsharedImage),
	mOwnsUnshared(false)
{
	if (mSharedImage)
		mSharedImage->mRefCount.fetch_add(1, std::memory_order_relaxed);
}

SharedImageRef::SharedImageRef() :
	mSharedImage(nullptr),
	mUnsharedImage(nullptr),
	mOwnsUnshared(false)
{
}

SharedImageRef::SharedImageRef(SharedImage* theSharedImage) :
	mSharedImage(theSharedImage),
	mUnsharedImage(nullptr),
	mOwnsUnshared(false)
{
	if (mSharedImage)
		mSharedImage->mRefCount.fetch_add(1, std::memory_order_relaxed);
}

SharedImageRef::~SharedImageRef()
{
	Release();
}

void SharedImageRef::Release()
{
	if (mOwnsUnshared)
		delete mUnsharedImage;
	mUnsharedImage = nullptr;

	if (mSharedImage)
	{
		if (mSharedImage->mRefCount.fetch_sub(1, std::memory_order_acq_rel) == 1)
		{
			if (gSexyAppBase)
				gSexyAppBase->mCleanupSharedImages = true;
		}
	}
	mSharedImage = nullptr;
}

SharedImageRef& SharedImageRef::operator=(const SharedImageRef& theSharedImageRef)
{
	if (this == &theSharedImageRef)
		return *this;

	Release();
	mSharedImage = theSharedImageRef.mSharedImage;
	if (mSharedImage)
		mSharedImage->mRefCount.fetch_add(1, std::memory_order_relaxed);
	mUnsharedImage = theSharedImageRef.mUnsharedImage;
	mOwnsUnshared = false;

	return *this;
}

SharedImageRef& SharedImageRef::operator=(SharedImage* theSharedImage)
{
	Release();
	mSharedImage = theSharedImage;
	if (mSharedImage)
		mSharedImage->mRefCount.fetch_add(1, std::memory_order_relaxed);
	return *this;
}

SharedImageRef& SharedImageRef::operator=(MemoryImage* theUnsharedImage)
{
	Release();
	mUnsharedImage = theUnsharedImage;
	return *this;
}

MemoryImage* SharedImageRef::operator->()
{
	return static_cast<MemoryImage*>(*this);
}

SharedImageRef::operator Image*()
{
	return static_cast<MemoryImage*>(*this);
}

SharedImageRef::operator MemoryImage*()
{
	if (mUnsharedImage)
		return mUnsharedImage;
	return static_cast<GLImage*>(*this);
}

SharedImageRef::operator GLImage*()
{
	return mSharedImage ? mSharedImage->mImage : nullptr;
}
