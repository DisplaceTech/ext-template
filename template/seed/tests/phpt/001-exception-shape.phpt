--TEST--
{{NAMESPACE}}Exception is registered and extends RuntimeException
--SKIPIF--
<?php
if (!extension_loaded('{{NAME}}')) {
    echo 'skip ext-{{NAME}} not loaded';
}
?>
--FILE--
<?php
use Displace\{{NAMESPACE}}\{{NAMESPACE}}Exception;

echo "registered: ", class_exists({{NAMESPACE}}Exception::class) ? "yes" : "no", "\n";
echo "extends_runtime: ",
    is_subclass_of({{NAMESPACE}}Exception::class, \RuntimeException::class) ? "yes" : "no",
    "\n";
?>
--EXPECT--
registered: yes
extends_runtime: yes
