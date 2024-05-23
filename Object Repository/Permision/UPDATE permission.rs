<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>UPDATE permission</name>
   <tag></tag>
   <elementGuidId>cfe1d56b-00a9-4e41-a429-6b1cf45cafcb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;${id_permission}\&quot;,\n  \&quot;project\&quot;: {\n    \&quot;id\&quot;: \&quot;${id_project}\&quot;,\n    \&quot;iconId\&quot;: \&quot;${iconid}\&quot;,\n    \&quot;name\&quot;: {\n      \&quot;th\&quot;: \&quot;${projectname_th}\&quot;,\n      \&quot;en\&quot;: \&quot;${projectname_en}\&quot;\n    }\n  },\n  \&quot;parcel\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;utilityBill\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;repair\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;cctv\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;vms\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;inviteMember\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: false,\n    \&quot;member\&quot;: false\n  },\n  \&quot;phoneBook\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: true,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;phoneBookEmergency\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: true,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;alarm\&quot;: {\n    \&quot;owner\&quot;: false,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: false,\n    \&quot;member\&quot;: false\n  },\n  \&quot;agent\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: false,\n    \&quot;member\&quot;: false\n  },\n  \&quot;contact\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: true,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;homeAutomation\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  },\n  \&quot;allService\&quot;: {\n    \&quot;owner\&quot;: true,\n    \&quot;contractor\&quot;: false,\n    \&quot;rental\&quot;: true,\n    \&quot;member\&quot;: true\n  }\n}&quot;,
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
      <webElementGuid>9063dcf2-984d-4096-8e18-9262e330da2c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>25d31421-147f-46fc-8b75-8632b6d5bb5d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>http://45.150.131.146:7090/api-backoffice/project-permission-menu</restUrl>
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
      <id>27dd1c4c-81d9-4a2d-8fec-a1289af9779c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_project</defaultValue>
      <description></description>
      <id>0d52b774-88d2-4052-8fe3-180633141ef8</id>
      <masked>false</masked>
      <name>id_project</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_permission</defaultValue>
      <description></description>
      <id>0332880b-5f13-4903-a253-c784e9dbd7f3</id>
      <masked>false</masked>
      <name>id_permission</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.iconid</defaultValue>
      <description></description>
      <id>a7bb09aa-5334-4e92-a9fe-64732a52cf49</id>
      <masked>false</masked>
      <name>iconid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.projectname_th</defaultValue>
      <description></description>
      <id>a59845c5-88cc-4495-bb26-6ec8d6d0de03</id>
      <masked>false</masked>
      <name>projectname_th</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.projectname_en</defaultValue>
      <description></description>
      <id>d4444726-f273-48e4-a148-082184164f67</id>
      <masked>false</masked>
      <name>projectname_en</name>
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
