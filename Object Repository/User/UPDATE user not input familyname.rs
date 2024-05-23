<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE user not input familyname</name>
   <tag></tag>
   <elementGuidId>227c7844-7760-49b4-b886-779375af81d1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id_user}\&quot;,\n  \&quot;name\&quot;: \&quot;testing2\&quot;,\n  \&quot;familyName\&quot;: null,\n  \&quot;ssoUserName\&quot;: \&quot;testing2\&quot;,\n  \&quot;phone\&quot;: \&quot;0123456789\&quot;,\n  \&quot;email\&quot;: \&quot;testing12@gmail.com\&quot;,\n  \&quot;password\&quot;: null,\n  \&quot;confirmPassword\&quot;: null,\n  \&quot;position\&quot;: \&quot;testing\&quot;,\n  \&quot;department\&quot;: \&quot;testing\&quot;,\n  \&quot;active\&quot;: true,\n  \&quot;role\&quot;: {\n    \&quot;id\&quot;: \&quot;${id_roles}\&quot;,\n    \&quot;name\&quot;: \&quot;${name_roles}\&quot;,\n    \&quot;createdAt\&quot;: \&quot;2021-12-07T07:54:38.325Z\&quot;,\n    \&quot;updatedAt\&quot;: \&quot;2022-05-26T03:24:28.000Z\&quot;,\n    \&quot;deletedAt\&quot;: null\n  },\n  \&quot;projects\&quot;: [],\n  \&quot;isReceiveContact\&quot;: true,\n  \&quot;isReceiveRepair\&quot;: true,\n  \&quot;isReceiveReport\&quot;: true\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>7017dfb2-9992-4808-8acb-03e93e3e6f96</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>54a5faf0-0abb-45a3-a5b0-50617baa3451</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/admin/user</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>fe207dcf-ad8a-459a-b90a-64fd94cbca90</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_user</defaultValue>
      <description></description>
      <id>a161e11d-f394-4e44-803f-867ee4eb134f</id>
      <masked>false</masked>
      <name>id_user</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_roles</defaultValue>
      <description></description>
      <id>a0195907-d03f-4a35-822b-6267a01e1080</id>
      <masked>false</masked>
      <name>id_roles</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.name_roles</defaultValue>
      <description></description>
      <id>803d58d3-f294-4ac2-bfad-b3503d60b84b</id>
      <masked>false</masked>
      <name>name_roles</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
