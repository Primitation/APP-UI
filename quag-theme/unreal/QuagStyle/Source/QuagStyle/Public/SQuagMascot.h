// Animated Quagsire. PRIVATE plugins only (DESIGN.md §6). Renders nothing if the
// plugin was shipped without Resources/Mascot, so it is always safe to use.
#pragma once

#include "CoreMinimal.h"
#include "Widgets/SCompoundWidget.h"

class SImage;

class QUAGSTYLE_API SQuagMascot : public SCompoundWidget
{
public:
	SLATE_BEGIN_ARGS(SQuagMascot)
		: _Size(160.f)
		, _Animate(true)
	{}
		/** Width in slate units (height keeps the sprite's aspect ratio). Keep 120–200. */
		SLATE_ARGUMENT(float, Size)
		/** false = show the first frame only (e.g. when the panel isn't the focus). */
		SLATE_ARGUMENT(bool, Animate)
	SLATE_END_ARGS()

	void Construct(const FArguments& InArgs);

private:
	EActiveTimerReturnType Advance(double InCurrentTime, float InDeltaTime);

	TSharedPtr<SImage> Image;
	int32 Frame = 0;
	float Elapsed = 0.f;
};
