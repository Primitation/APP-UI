#include "QuagStyle.h"
#include "Modules/ModuleManager.h"

class FQuagStyleModule : public IModuleInterface
{
public:
	virtual void StartupModule() override  { FQuagStyle::Initialize(); }
	virtual void ShutdownModule() override { FQuagStyle::Shutdown(); }
};

IMPLEMENT_MODULE(FQuagStyleModule, QuagStyle)
