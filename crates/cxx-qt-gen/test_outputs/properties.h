#pragma once

#include <cxx-qt-common/cxxqt_locking.h>
#include <cxx-qt-common/cxxqt_maybelockguard.h>
#include <cxx-qt-common/cxxqt_signalhandler.h>
#include <cxx-qt-common/cxxqt_type.h>

namespace cxx_qt::my_object {
class MyObject;

} // namespace cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
using MyObjectCxxQtSignalHandlerprimitiveChanged =
  ::rust::cxxqtlib1::SignalHandler<
    struct MyObjectCxxQtSignalParamsprimitiveChanged*>;
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
using MyObjectCxxQtSignalHandlertrivialChanged =
  ::rust::cxxqtlib1::SignalHandler<
    struct MyObjectCxxQtSignalParamstrivialChanged*>;
} // namespace rust::cxxqtgen1::cxx_qt::my_object

#include "cxx-qt-gen/ffi.cxx.h"

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_primitiveChangedConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::
    MyObjectCxxQtSignalHandlerprimitiveChanged closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_trivialChangedConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlertrivialChanged
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , public ::rust::cxxqtlib1::CxxQtType<MyObjectRust>
  , public ::rust::cxxqtlib1::CxxQtLocking
{
  Q_OBJECT
public:
  Q_PROPERTY(::std::int32_t primitive READ getPrimitive WRITE setPrimitive
               NOTIFY primitiveChanged)
  Q_PROPERTY(
    QPoint trivial READ getTrivial WRITE setTrivial NOTIFY trivialChanged)

  virtual ~MyObject() = default;

public:
  ::std::int32_t const& getPrimitive() const;
  Q_SLOT void setPrimitive(::std::int32_t const& value);
  QPoint const& getTrivial() const;
  Q_SLOT void setTrivial(QPoint const& value);
  Q_SIGNAL void primitiveChanged();
  Q_SIGNAL void trivialChanged();
  explicit MyObject(QObject* parent = nullptr);

private:
  ::std::int32_t const& getPrimitiveWrapper() const noexcept;
  void setPrimitiveWrapper(::std::int32_t value) noexcept;
  QPoint const& getTrivialWrapper() const noexcept;
  void setTrivialWrapper(QPoint value) noexcept;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)
